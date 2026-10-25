//! rig-harness plugins for daily use, after Pi's coding agent. Each is a
//! separate plugin type, so a recipe selects them one by one:
//!
//! - [`search::SearchTools`] adds Rig's `search` and `list` tools, the
//!   equivalents of Pi's `grep`, `find` and `ls`.
//! - [`templates::PromptTemplatesPlugin`] makes Markdown prompt templates
//!   `/name` commands, with Pi's argument substitutions.
//! - [`session::SessionInfoPlugin`] adds `/session`.
//! - [`session::CopyPlugin`] adds `/copy`, the last reply to the clipboard.
//! - [`session::ExportPlugin`] adds `/export [path]`, as HTML or JSONL.
//! - [`context::ContextFilesPlugin`] reads AGENTS.md/CLAUDE.md and the
//!   SYSTEM.md/APPEND_SYSTEM.md prompt files as Pi does.
//! - [`model::PiModelControlsPlugin`] adds Pi's `/thinking`, `--thinking`
//!   and model and reasoning cycling.
//! - [`trust::TrustPlugin`] decides project trust as Pi does, and
//!   [`skills::TrustedSkillsPlugin`] is Rig's skills without untrusted
//!   project skills.
//! - [`branch::BranchSummaryPlugin`] summarizes what a rewind leaves behind
//!   into the fork, as Pi's branch summary does.

pub mod branch;
pub mod context;
pub mod model;
pub mod search;
pub mod session;
pub mod shell;
pub mod skills;
pub mod templates;
pub mod trust;
