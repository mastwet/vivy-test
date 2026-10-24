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

pub mod search;
pub mod session;
pub mod shell;
pub mod templates;
