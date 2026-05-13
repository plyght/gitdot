// Several modules (git, store, util, RunnerConfig, parts of GitdotClient) are
// kept on disk for later re-enable as commands ship. Suppress the resulting
// dead-code noise crate-wide until those commands are wired back in.
// TODO: remove it
#![allow(dead_code)]

pub mod bootstrap;
mod cli;
mod client;
mod command;
mod config;
mod git;
mod store;
mod util;

pub use cli::run;
pub use command::Args;
