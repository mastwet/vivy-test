//! An auxiliary persistent CPython tool. Native coding tools remain available.

use rig_harness::prelude::*;
use rig_harness::rig_core::{
    serve::{ErasedHandler, adapters::ToolAdapter},
    tool::{ToolContext, ToolExecutionError},
};
use rig_harness::rig_ecs::effects::Handler;
use rig_harness::rig_tools::{Live, OutputLimit};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{ChildStdin, Stdio};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc,
};
use std::time::Instant;
use vivy_process::{Control, OwnedChild};

const WORKER: &str = include_str!("worker.py");

#[derive(Resource, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct PythonSettings {
    executable: String,
}
impl Default for PythonSettings {
    fn default() -> Self {
        Self {
            executable: "python".into(),
        }
    }
}

#[derive(Resource, Default)]
struct Workbenches(HashMap<Entity, Arc<Workbench>>);

#[derive(Default)]
pub struct PythonPlugin;
impl Plugin for PythonPlugin {
    fn build(&self, app: &mut App) {
        app.add_setting::<PythonSettings>("python")
            .init_resource::<Workbenches>();
        app.add_tool_per_call(
            PythonTool::for_directory(PathBuf::new()),
            ToolOptions::default(),
            |place| PythonTool::for_directory(place.base),
        );
        app.add_systems(AgentPass, bind_calls.in_set(PrepareSystems::Edit));
        app.add_systems(First, prune_workers);
        app.add_systems(
            Last,
            stop_workers
                .in_set(OnAppExitSystems)
                .run_if(on_message::<rig_harness::AppExit>),
        );
    }
}

fn bind_calls(
    mut calls: Query<
        (Entity, &ToolRequest, &ForAgent, &mut CallHandler),
        (ToolCalls, Added<Admitted>),
    >,
    places: PerCallTools,
    workspace: Res<ProjectWorkspace>,
    settings: Res<PythonSettings>,
    mut workers: ResMut<Workbenches>,
    wake: Res<Wake>,
    mut commands: Commands,
    scratch: Option<Res<Scratch>>,
) {
    for (call, request, agent, mut handler) in &mut calls {
        if request.call.function.name.as_str() != PythonTool::NAME {
            continue;
        }
        let place = places.place(Some(agent.0), workspace.0.clone());
        let state = workers.0.entry(agent.0).or_default().clone();
        let (output, live) = CallOutput::new(&wake);
        commands.entity(call).insert(output);
        let tool = PythonTool {
            state,
            dir: place.base,
            settings: settings.clone(),
            live: Some(live),
            spill: scratch.as_ref().and_then(|scratch| scratch.spill()),
        };
        *handler = CallHandler(Handler(ErasedHandler::new(ToolAdapter::new(tool))));
    }
}

fn prune_workers(mut workers: ResMut<Workbenches>, agents: Query<(), With<Agent>>) {
    workers.0.retain(|agent, worker| {
        if agents.contains(*agent) {
            worker.refresh_credential_policy();
            true
        } else {
            worker.stop();
            false
        }
    });
}
fn stop_workers(workers: Res<Workbenches>) {
    for worker in workers.0.values() {
        worker.stop();
    }
}

#[derive(Default)]
struct Workbench {
    worker: Mutex<Option<Worker>>,
    control: Mutex<Option<Control>>,
    policy_generation: AtomicU64,
}
impl Workbench {
    fn refresh_credential_policy(&self) {
        let generation = vivy_process::credential_policy_generation();
        if self.policy_generation.load(Ordering::Acquire) != generation {
            self.stop();
            if let Ok(mut worker) = self.worker.try_lock() {
                *worker = None;
                self.policy_generation.store(generation, Ordering::Release);
            }
        }
    }
    fn stop(&self) {
        if let Some(control) = self
            .control
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            control.kill();
        }
    }
}
impl Drop for Workbench {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Default)]
struct Cancellation {
    control: Mutex<Option<Control>>,
    cancelled: AtomicBool,
}
impl Cancellation {
    fn bind(&self, control: Control) {
        let mut slot = self.control.lock().unwrap_or_else(|e| e.into_inner());
        if self.cancelled.load(Ordering::SeqCst) {
            control.kill();
        }
        *slot = Some(control);
    }
    fn stop(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        if let Some(control) = self
            .control
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            control.kill();
        }
    }
}
struct CancelOnDrop(Arc<Cancellation>, bool);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        if self.1 {
            self.0.stop();
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PythonArgs {
    code: String,
    timeout_secs: Option<u64>,
}

#[derive(Clone)]
struct PythonTool {
    state: Arc<Workbench>,
    dir: PathBuf,
    settings: PythonSettings,
    live: Option<Live>,
    spill: Option<Spill>,
}
impl PythonTool {
    fn for_directory(dir: PathBuf) -> Self {
        Self {
            state: Arc::default(),
            dir,
            settings: PythonSettings::default(),
            live: None,
            spill: None,
        }
    }
}
impl Tool for PythonTool {
    const NAME: &'static str = "python_repl";
    type Args = PythonArgs;
    type Output = String;
    type Error = ToolExecutionError;
    fn description(&self) -> String {
        "Execute Python in a persistent, auxiliary workbench. Variables and imports persist between calls for this agent; top-level await works. Use the normal coding tools for file edits and Git Bash commands. Python state is NOT saved across exit/session switches. A timeout or cancellation stops the worker and all descendants and resets its variables. Default timeout: 60 seconds. No automatic package installation or background tasks.".into()
    }
    fn parameters(&self) -> Value {
        json!({"type":"object", "properties":{"code":{"type":"string"},"timeout_secs":{"type":"integer","minimum":1}},"required":["code"],"additionalProperties":false})
    }
    async fn call(&self, _: &mut ToolContext, args: PythonArgs) -> Result<String, Self::Error> {
        let cancellation = Arc::<Cancellation>::default();
        let mut guard = CancelOnDrop(cancellation.clone(), true);
        let tool = self.clone();
        let result = blocking(move || tool.execute(args, &cancellation)).await;
        guard.1 = false;
        let (text, error) = result?;
        if error {
            Err(ToolExecutionError::other(text))
        } else {
            Ok(text)
        }
    }
}

impl PythonTool {
    fn execute(
        &self,
        args: PythonArgs,
        cancellation: &Cancellation,
    ) -> Result<(String, bool), ToolExecutionError> {
        let error = |why| {
            ToolExecutionError::other(format!("Python workbench stopped; variables reset: {why}"))
        };
        let mut slot = self.state.worker.lock().unwrap_or_else(|e| e.into_inner());
        let generation = vivy_process::credential_policy_generation();
        if self.state.policy_generation.load(Ordering::Acquire) != generation {
            *slot = None;
            self.state
                .policy_generation
                .store(generation, Ordering::Release);
        }
        let fresh = slot.is_none();
        let mut worker = match slot.take() {
            Some(worker) => worker,
            None => {
                Worker::start(&self.settings.executable, &self.dir, cancellation).map_err(error)?
            }
        };
        cancellation.bind(worker.process.control());
        *self.state.control.lock().unwrap_or_else(|e| e.into_inner()) =
            Some(worker.process.control());
        let timeout = Duration::from_secs(args.timeout_secs.unwrap_or(60).max(1));
        let reply = worker
            .execute(&args.code, timeout, self.live.as_ref())
            .map_err(error)?;
        *slot = Some(worker);
        let (kept, dropped) = OutputLimit::default().tail(&reply.0);
        let mut text = if fresh {
            "[Fresh Python workbench; previous process/session variables are not restored.]\n"
                .to_owned()
        } else {
            String::new()
        };
        text.push_str(kept);
        if dropped > 0 {
            if let Some(spill) = &self.spill {
                let (path, mut file) = spill.create("python").map_err(|e| error(e.to_string()))?;
                file.write_all(reply.0.as_bytes())
                    .map_err(|e| error(e.to_string()))?;
                text.push_str(&format!("\n[full output: {}]", path.display()));
            } else {
                text.push_str("\n[output truncated]");
            }
        }
        Ok((text, reply.1))
    }
}

struct Worker {
    process: OwnedChild,
    input: ChildStdin,
    events: mpsc::Receiver<Result<Value, String>>,
    sequence: u64,
}
impl Worker {
    fn start(
        executable: &str,
        dir: &std::path::Path,
        cancellation: &Cancellation,
    ) -> Result<Self, String> {
        let mut command = vivy_process::command(executable);
        command
            .args(["-I", "-u", "-c", WORKER])
            .current_dir(dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut process = OwnedChild::spawn(&mut command).map_err(|e| {
            format!(
                "could not start {executable}; install Python 3.11+ or set [python].executable: {e}"
            )
        })?;
        cancellation.bind(process.control());
        let input = process.child.stdin.take().ok_or("missing worker input")?;
        let output = process.child.stdout.take().ok_or("missing worker output")?;
        let mut stderr = process.child.stderr.take().ok_or("missing worker errors")?;
        let (sender, events) = mpsc::sync_channel::<Result<Value, String>>(64);
        let errors = sender.clone();
        std::thread::spawn(move || {
            let mut text = String::new();
            let _ = stderr.by_ref().take(8192).read_to_string(&mut text);
            if !text.trim().is_empty() {
                let _ = errors.send(Err(text));
            }
        });
        std::thread::spawn(move || {
            let mut reader = BufReader::new(output);
            loop {
                let mut bytes = Vec::new();
                match reader
                    .by_ref()
                    .take(2 * 1024 * 1024)
                    .read_until(b'\n', &mut bytes)
                {
                    Ok(0) => break,
                    Ok(_) => {
                        let event = serde_json::from_slice(&bytes)
                            .map_err(|e| format!("invalid worker protocol: {e}"));
                        if sender.send(event).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = sender.send(Err(e.to_string()));
                        break;
                    }
                }
            }
        });
        let ready = events
            .recv_timeout(Duration::from_secs(10))
            .map_err(|e| format!("worker startup failed: {e}"))??;
        if ready["event"] != "ready"
            || ready["python"][0].as_u64().unwrap_or(0) < 3
            || (ready["python"][0] == 3 && ready["python"][1].as_u64().unwrap_or(0) < 11)
        {
            return Err("Python 3.11 or newer is required".into());
        }
        Ok(Self {
            process,
            input,
            events,
            sequence: 0,
        })
    }

    fn execute(
        &mut self,
        code: &str,
        timeout: Duration,
        live: Option<&Live>,
    ) -> Result<(String, bool), String> {
        self.sequence += 1;
        writeln!(self.input, "{}", json!({"id":self.sequence,"code":code}))
            .and_then(|_| self.input.flush())
            .map_err(|e| e.to_string())?;
        let deadline = Instant::now() + timeout;
        let mut output = String::new();
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err("execution timed out".into());
            }
            let event = self
                .events
                .recv_timeout(remaining)
                .map_err(|e| format!("execution interrupted, timed out or worker exited: {e}"))??;
            if event["id"].as_u64() != Some(self.sequence) {
                return Err("worker response did not match the current call".into());
            }
            if event["event"] == "done" {
                return Ok((output, event["ok"] != true));
            }
            if let Some(text) = event["text"].as_str() {
                if output.len() < 10 * 1024 * 1024 {
                    output.push_str(text);
                    if let Some(live) = live {
                        live.line(text);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn credential_policy_change_restarts_persistent_workers() {
        if std::env::var_os("VIVY_POLICY_TEST_CHILD").is_none() {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "tests::credential_policy_change_restarts_persistent_workers",
                    "--nocapture",
                ])
                .env("VIVY_POLICY_TEST_CHILD", "1")
                .status()
                .unwrap();
            assert!(status.success());
            return;
        }
        let tool = PythonTool::for_directory(std::env::temp_dir());
        call(&tool, "marker_before_policy = 42", None).unwrap();
        vivy_process::register_credential_variables(&["VIVY_POLICY_EXPANSION_FIXTURE".into()]);
        let result = call(
            &tool,
            "'STALE_POLICY' if 'marker_before_policy' in globals() else 'FRESH_POLICY'",
            None,
        )
        .unwrap();
        assert!(result.contains("FRESH_POLICY"), "{result}");
    }
    use super::*;
    fn call(
        tool: &PythonTool,
        code: &str,
        timeout_secs: Option<u64>,
    ) -> Result<String, ToolExecutionError> {
        futures::executor::block_on(tool.call(
            &mut ToolContext::default(),
            PythonArgs {
                code: code.into(),
                timeout_secs,
            },
        ))
    }

    #[test]
    fn persistent_workbench_and_independent_agents() {
        let tool = PythonTool::for_directory(std::env::temp_dir());
        call(&tool, "value = 40", None).unwrap();
        assert!(
            call(
                &tool,
                "import asyncio\nawait asyncio.sleep(0)\nvalue + 2",
                None
            )
            .unwrap()
            .contains("42")
        );
        assert!(call(&tool, "raise ValueError('expected')", None).is_err());
        assert!(call(&tool, "value", None).unwrap().contains("40"));
        let other = PythonTool::for_directory(std::env::temp_dir());
        assert!(call(&other, "value", None).is_err());
    }

    #[test]
    fn a_large_unicode_result_does_not_reset_the_workbench() {
        let tool = PythonTool::for_directory(std::env::temp_dir());
        call(&tool, "s = chr(0x1F600) * 600000; s", None).unwrap();
        assert!(call(&tool, "len(s)", None).unwrap().contains("600000"));
    }

    #[test]
    fn timeout_resets_variables_and_the_next_call_recovers() {
        let tool = PythonTool::for_directory(std::env::temp_dir());
        let start = std::time::Instant::now();
        assert!(
            call(&tool, "value = 42\nimport time\ntime.sleep(30)", Some(1))
                .unwrap_err()
                .to_string()
                .contains("reset")
        );
        assert!(start.elapsed().as_secs() < 5);
        assert!(
            call(
                &tool,
                "'fresh' if 'value' not in globals() else 'stale'",
                None
            )
            .unwrap()
            .contains("fresh")
        );
    }

    #[test]
    fn dropping_a_tool_future_stops_the_worker() {
        use futures::{FutureExt, poll};
        let tool = PythonTool::for_directory(std::env::temp_dir());
        call(&tool, "value = 1", None).unwrap();
        futures::executor::block_on(async {
            let mut context = ToolContext::default();
            let mut future = tool
                .call(
                    &mut context,
                    PythonArgs {
                        code: "import time\ntime.sleep(60)".into(),
                        timeout_secs: None,
                    },
                )
                .boxed();
            assert!(poll!(future.as_mut()).is_pending());
            std::thread::sleep(std::time::Duration::from_millis(100));
            drop(future);
        });
        let start = std::time::Instant::now();
        assert!(
            call(
                &tool,
                "'fresh' if 'value' not in globals() else 'stale'",
                None
            )
            .unwrap()
            .contains("fresh")
        );
        assert!(start.elapsed().as_secs() < 5);
    }
}
