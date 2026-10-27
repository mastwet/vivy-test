//! Explicit preparation is a cancellable owned child, never an implicit tool side effect.
use super::*;
use std::path::Path;
use vivy_basics::skills::{TrustedSkillInventory, refresh_inventory};
mod status;

const CORE: &str = include_str!("../requirements-core.lock");
const MEMORY: &str = include_str!("../requirements-memory.lock");

struct Preparation {
    cancel: Arc<Cancellation>,
    result: Mutex<mpsc::Receiver<Result<String, String>>>,
}
#[derive(Resource, Default)]
struct Preparations(
    HashMap<Entity, Preparation>,
    std::collections::HashSet<Entity>,
    u64,
);

pub(super) fn install(app: &mut App) {
    app.init_resource::<Preparations>()
        .add_command("python", "Python status | prepare | reset | close", command)
        .add_systems(First, (poll, status::publish).chain())
        .add_systems(
            Last,
            stop_preparations
                .in_set(OnAppExitSystems)
                .run_if(on_message::<rig_harness::AppExit>),
        )
        .add_observer(cancel_turn);
}

pub(super) fn skill_metadata(inventory: Option<&TrustedSkillInventory>) -> Value {
    json!(
        inventory
            .map(|i| i
                .0
                .iter()
                .filter(|s| !s.manual)
                .map(|s| json!({"name":s.name,"description":s.description,"file":s.file}))
                .collect::<Vec<_>>())
            .unwrap_or_default()
    )
}

pub(super) fn prepared_python(root: &Path) -> Result<Option<String>, String> {
    let file = root.join("current.json");
    let bytes = match std::fs::read(&file) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("Cannot read prepared Python manifest: {e}")),
    };
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("Invalid Python manifest; run /python prepare: {e}"))?;
    let path = value["executable"]
        .as_str()
        .ok_or("Python manifest has no executable; run /python prepare")?;
    if !Path::new(path).is_file() {
        return Err("Prepared Python executable is missing; run /python prepare".into());
    }
    Ok(Some(path.to_owned()))
}

fn command(In(args): In<CommandArgs>, world: &mut World) {
    let action = args.args.trim();
    let root = world.resource::<ProjectRoot>().0.clone();
    if matches!(action, "reset" | "close" | "prepare" | "status" | "") {
        let mut jobs = world.resource_mut::<Preparations>();
        jobs.2 = jobs.2.wrapping_add(1);
        if matches!(action, "reset" | "close" | "prepare") {
            jobs.1.remove(&args.agent);
        }
    }
    if action == "reset" || action == "prepare" {
        if world.contains_resource::<TrustedSkillInventory>() {
            refresh_inventory(world);
        }
    }
    if action == "close" || action == "reset" {
        if let Some(job) = world.resource_mut::<Preparations>().0.remove(&args.agent) {
            job.cancel.stop();
        }
        if let Some(worker) = world.resource_mut::<Workbenches>().0.remove(&args.agent) {
            worker.retire();
        }
        world.write_message(Notice::info(args.agent, "Python closed; variables reset. Explicit artifacts and committed memories are preserved. Next use starts fresh; changed dependencies require /python prepare."));
    } else if action == "prepare" {
        if world.resource::<Preparations>().0.contains_key(&args.agent) {
            world.write_message(Notice::info(
                args.agent,
                "Python preparation is already running.",
            ));
            return;
        }
        let mut settings = world.resource::<PythonSettings>().clone();
        settings.skills = skill_metadata(world.get_resource::<TrustedSkillInventory>());
        let cancel = Arc::<Cancellation>::default();
        let token = cancel.clone();
        let (sender, receiver) = mpsc::channel();
        let wake = world.resource::<Wake>().clone();
        std::thread::spawn(move || {
            let _ = sender.send(prepare(&root, &settings, &token));
            wake.wake();
        });
        world.resource_mut::<Preparations>().0.insert(
            args.agent,
            Preparation {
                cancel,
                result: Mutex::new(receiver),
            },
        );
        world.write_message(Notice::info(args.agent, "Preparing a new Python environment. Use /python close to cancel; existing workers keep their environment until reset."));
    } else if action.is_empty() || action == "status" {
        let preparing = world.resource::<Preparations>().0.contains_key(&args.agent);
        let active = world.resource::<Workbenches>().0.contains_key(&args.agent);
        let prepared = match prepared_python(&root.join(".vivy/workbench")) {
            Ok(value) => value.is_some().to_string(),
            Err(error) => error,
        };
        world.write_message(Notice::info(args.agent, format!("Python: prepared={prepared}, agent workbench allocated={active}, preparing={preparing}. Inspect actual bindings with workbench.list_skills(); MCP opens lazily.")));
    } else {
        world.write_message(Notice::error(
            args.agent,
            "Use /python status, prepare, reset or close.",
        ));
    }
}

fn prepare(dir: &Path, settings: &PythonSettings, cancel: &Cancellation) -> Result<String, String> {
    let root = dir.join(".vivy/workbench");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let source =
        worker::package(&root.join(format!("prepare-{}-{timestamp}", std::process::id())))?;
    let lock = source.join("requirements.lock");
    std::fs::write(&lock, if settings.memory { MEMORY } else { CORE })
        .map_err(|e| e.to_string())?;
    let memory_root = settings
        .memory_root
        .clone()
        .unwrap_or_else(|| root.join("native-memory"));
    let code = format!(
        "import sys,os,json;sys.path.insert(0,{});from vivy_workbench.prepare import prepare;print(prepare({}, {}, python=sys.executable, skills=json.loads(os.environ['VIVY_WORKBENCH_SKILLS']), native_memory={}, memory_root={}))",
        json!(source),
        json!(root),
        json!(lock),
        if settings.memory { "True" } else { "False" },
        json!(memory_root)
    );
    let mut command = vivy_process::command(&settings.executable);
    // Preparation is noninteractive. Inheriting stdin can block spawning while
    // the JSON input reader owns Rust's stdin lock, and would expose user input.
    command
        .args(["-I", "-u", "-c", &code])
        .current_dir(dir)
        .env("VIVY_WORKBENCH_SKILLS", settings.skills.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = OwnedChild::spawn(&mut command).map_err(|e| e.to_string())?;
    cancel.bind(child.control());
    let errors = child.child.stderr.take().ok_or("missing prepare stderr")?;
    let drain = std::thread::spawn(move || worker::diagnostic_tail(errors));
    let mut output = child.child.stdout.take().ok_or("missing prepare stdout")?;
    std::io::copy(&mut output, &mut std::io::sink()).map_err(|e| e.to_string())?;
    let status = child.child.wait().map_err(|e| e.to_string())?;
    let diagnostic = drain.join().unwrap_or_default();
    if !status.success() {
        return Err(format!(
            "Python preparation failed ({status}); previous environment preserved. {diagnostic}"
        ));
    }
    Ok("Python environment prepared. /python reset adopts it; installed packages and live bindings are separate.".into())
}

fn poll(
    mut jobs: ResMut<Preparations>,
    agents: Query<(), With<Agent>>,
    mut notices: MessageWriter<Notice>,
) {
    let Preparations(running, failures, revision) = &mut *jobs;
    failures.retain(|agent| agents.contains(*agent));
    running.retain(|agent, job| {
        if !agents.contains(*agent) {
            job.cancel.stop();
            return false;
        }
        match job
            .result
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .try_recv()
        {
            Ok(Ok(text)) => {
                *revision = revision.wrapping_add(1);
                failures.remove(agent);
                notices.write(Notice::info(*agent, text));
                false
            }
            Ok(Err(text)) => {
                *revision = revision.wrapping_add(1);
                if !job.cancel.cancelled.load(Ordering::SeqCst) {
                    failures.insert(*agent);
                }
                notices.write(Notice::error(*agent, text));
                false
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                failures.insert(*agent);
                notices.write(Notice::error(*agent, "Python preparation worker exited."));
                false
            }
            Err(mpsc::TryRecvError::Empty) => true,
        }
    });
}
fn cancel_turn(event: On<StopTurn>, jobs: Res<Preparations>, workers: Res<Workbenches>) {
    if let Some(job) = jobs.0.get(&event.entity) {
        job.cancel.stop();
    }
    if let Some(worker) = workers.0.get(&event.entity) {
        worker.stop();
    }
}
fn stop_preparations(jobs: Res<Preparations>) {
    for job in jobs.0.values() {
        job.cancel.stop();
    }
}
