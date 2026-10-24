//! Launches the compiled species and honors Rig's existing session-switch protocol.

use crate::Result;
use rig_harness_protocol::{AgentDescription, Home, RELOAD_EXIT_CODE, SessionId, StartReason, env};
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(binary: &Path, args: &[String]) -> Result<u8> {
    if !binary.is_file() {
        return Err(format!("the coding species is not built at {}; run `vivy build recipes/coding-agent.yaml` first", binary.display()).into());
    }
    let home = Home::from_env();
    let here = std::env::current_dir()?;
    let run = home.run(std::process::id());
    fs::create_dir_all(run.path())?;
    let run_lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(run.lock())?;
    run_lock.try_lock()?;
    let description_file = run.path().join("description");
    let mut describe = Command::new(binary);
    for (name, _) in std::env::vars_os() {
        if name
            .to_string_lossy()
            .to_ascii_uppercase()
            .starts_with("RIG_SECRET_")
        {
            describe.env_remove(name);
        }
    }
    for name in env::AGENT_ONLY {
        describe.env_remove(name);
    }
    let status = describe
        .env(env::DESCRIBE, &description_file)
        .stdout(Stdio::null())
        .status()?;
    if !status.success() {
        return Err("the compiled species could not describe its command line".into());
    }
    let report = AgentDescription::parse(&fs::read_to_string(&description_file)?);
    let invocation = report.invocation(args)?;
    let resume = report.resume(args);
    let mut session = resume.map(|name| home.find(&name)).transpose()?;
    let mut reason = if session.is_some() {
        StartReason::Resumed
    } else {
        StartReason::New
    };
    // The agent receives console interrupts itself and performs its own cleanup.
    ctrlc::set_handler(|| {})?;
    loop {
        let id = session.take().unwrap_or_else(SessionId::generate);
        let dir = home.session(&id);
        let _session_lock = dir.hold()?.ok_or("this session is already running")?;
        let cwd = if reason == StartReason::Resumed {
            dir.working_directory()
                .ok_or("the resumed session's working directory is unavailable")?
        } else {
            here.clone()
        };
        if reason == StartReason::New {
            fs::write(dir.directory(), cwd.to_string_lossy().as_bytes())?;
        }
        if dir.switch().is_file() {
            fs::remove_file(dir.switch())?;
        }
        let mut command = Command::new(binary);
        for name in env::AGENT_ONLY {
            command.env_remove(name);
        }
        let status = command
            .args(invocation.to_args())
            .current_dir(&cwd)
            .env(env::HOME, home.root())
            .env(env::SESSION, id.as_str())
            .env(env::LAUNCHER, std::env::current_exe()?)
            .env(env::RUN, run.path())
            .env(env::START, reason.as_str())
            .status()?;
        if status.code() == Some(i32::from(RELOAD_EXIT_CODE)) && dir.switch().is_file() {
            // A print prompt has already been consumed. Replaying `/new` on
            // every restart would create an endless chain of sessions.
            if invocation.is_headless() {
                return Ok(RELOAD_EXIT_CODE);
            }
            let target = fs::read_to_string(dir.switch())?;
            fs::remove_file(dir.switch())?;
            session = if target.trim().is_empty() {
                None
            } else {
                Some(home.find(target.trim())?)
            };
            reason = if session.is_some() {
                StartReason::Resumed
            } else {
                StartReason::New
            };
            continue;
        }
        return Ok(status
            .code()
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or(130));
    }
}
