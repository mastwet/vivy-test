//! `vivy`: builds a Rig agent from a YAML recipe.
//!
//! ```text
//! vivy generate <recipe.yaml>   write .generated/<name>/
//! vivy build    <recipe.yaml>   generate, then cargo build; prints the binary
//! ```
//!
//! The agent binary is then run directly; its runtime settings, such as the
//! gateway, are in `RIG_HOME/settings.toml`.

mod generate;
mod launch;
mod recipe;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use recipe::Recipe;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const USAGE: &str = "usage: vivy [--resume <id>] [--model <model>] [-p prompt | --json]\n       vivy generate <recipe.yaml> | vivy build <recipe.yaml>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [command, recipe] if command == "generate" => Agent::load(Path::new(recipe))
            .and_then(|agent| agent.generate().map(|()| agent.package)),
        [command, recipe] if command == "build" => Agent::load(Path::new(recipe))
            .and_then(|agent| agent.generate().and_then(|()| agent.build())),
        [flag] if flag == "--help" || flag == "-h" => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        [command, ..] if command == "build" || command == "generate" => Err(USAGE.into()),
        _ => {
            let result = Agent::load(&repository_root().join("recipes/coding-agent.yaml"))
                .and_then(|agent| launch::run(&agent.binary(), &args));
            return match result {
                Ok(code) => ExitCode::from(code),
                Err(error) => {
                    eprintln!("vivy: {error}");
                    ExitCode::FAILURE
                }
            };
        }
    };
    match result {
        Ok(path) => {
            println!("{}", path.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("vivy: {error}");
            ExitCode::FAILURE
        }
    }
}

/// One recipe and where its generated package, lock and binary live.
struct Agent {
    recipe: Recipe,
    origin: String,
    directory: PathBuf,
    /// The lock kept next to the recipe, so a build is reproducible.
    lock: PathBuf,
    package: PathBuf,
    target: PathBuf,
}

impl Agent {
    fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read recipe {}: {error}", path.display()))?;
        let recipe =
            Recipe::parse(&text).map_err(|error| format!("recipe {}: {error}", path.display()))?;
        let absolute = std::path::absolute(path)?;
        // Generated packages share one target directory, so a changed
        // selection rebuilds only what changed.
        let generated = repository_root().join(".generated");
        Ok(Self {
            package: generated.join(&recipe.name),
            target: target_directory(std::env::var_os("CARGO_TARGET_DIR"))?,
            lock: path.with_extension("Cargo.lock"),
            origin: path.display().to_string(),
            directory: absolute.parent().unwrap_or(Path::new("/")).to_path_buf(),
            recipe,
        })
    }

    fn generate(&self) -> Result<()> {
        generate::write(&self.package, &self.recipe, &self.origin, &self.directory)?;
        if self.lock.is_file() {
            let lock = fs::read_to_string(&self.lock)?;
            generate::write_if_changed(&self.package.join("Cargo.lock"), &lock)?;
        }
        Ok(())
    }

    /// Builds the generated package and returns the agent binary.
    fn build(&self) -> Result<PathBuf> {
        let mut command = cargo_command(
            std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
            &self.package,
            &self.target,
            std::env::vars_os().map(|(key, _)| key),
        );
        let names =
            vivy_process::variables_from_file(&rig_harness_protocol::Home::from_env().settings());
        vivy_process::filter_credentials(&mut command, &names);
        let status = command
            .status()
            .map_err(|error| format!("cannot start cargo: {error}"))?;
        if !status.success() {
            return Err(format!(
                "cargo build of {} failed ({status}); its output above names the cause",
                self.package.display()
            )
            .into());
        }
        // Keep the resolved versions with the recipe.
        let lock = fs::read_to_string(self.package.join("Cargo.lock"))?;
        generate::write_if_changed(&self.lock, &lock)?;
        Ok(self.binary())
    }

    fn binary(&self) -> PathBuf {
        self.target.join("debug").join(format!(
            "{}{}",
            self.recipe.name,
            std::env::consts::EXE_SUFFIX
        ))
    }
}

fn target_directory(override_path: Option<std::ffi::OsString>) -> std::io::Result<PathBuf> {
    override_path.map_or_else(
        || Ok(repository_root().join(".generated/target")),
        std::path::absolute,
    )
}

fn cargo_command(
    cargo: impl AsRef<std::ffi::OsStr>,
    package: &Path,
    target: &Path,
    variables: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>,
) -> Command {
    let mut command = Command::new(cargo);
    command
        .arg("build")
        .current_dir(package)
        .env("CARGO_TARGET_DIR", target);
    for name in variables {
        if name
            .as_ref()
            .to_string_lossy()
            .to_ascii_uppercase()
            .starts_with("RIG_SECRET_")
        {
            command.env_remove(name);
        }
    }
    command
}

/// This repository's root: `vivy` is run from its own checkout.
fn repository_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .nth(2)
        .unwrap_or(manifest)
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_removes_secrets() {
        let command = cargo_command(
            "cargo",
            Path::new("."),
            Path::new("target"),
            ["RIG_SECRET_GATEWAY", "PATH"],
        );
        assert!(
            command
                .get_envs()
                .any(|(key, value)| key == "RIG_SECRET_GATEWAY" && value.is_none())
        );
        assert!(!command.get_envs().any(|(key, _)| key == "PATH"));
    }

    #[test]
    fn target_override_is_resolved_before_changing_directory() {
        let cwd = std::env::current_dir().unwrap();
        assert_eq!(
            target_directory(Some("short-target".into())).unwrap(),
            cwd.join("short-target")
        );
    }
}
