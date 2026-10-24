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
mod recipe;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use recipe::Recipe;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const USAGE: &str = "usage: vivy generate <recipe.yaml> | vivy build <recipe.yaml>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [command, recipe] if command == "generate" => Agent::load(Path::new(recipe))
            .and_then(|agent| agent.generate().map(|()| agent.package)),
        [command, recipe] if command == "build" => Agent::load(Path::new(recipe))
            .and_then(|agent| agent.generate().and_then(|()| agent.build())),
        _ => Err(USAGE.into()),
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
            target: generated.join("target"),
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
        let status = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .arg("build")
            .current_dir(&self.package)
            .env("CARGO_TARGET_DIR", &self.target)
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
        Ok(self.target.join("debug").join(format!(
            "{}{}",
            self.recipe.name,
            std::env::consts::EXE_SUFFIX
        )))
    }
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
