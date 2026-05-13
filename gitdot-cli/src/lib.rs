// Several modules are kept on disk for later re-enable as commands ship.
// Suppress the resulting dead-code noise crate-wide until those commands are
// wired back in.
// TODO: remove it
#![allow(dead_code)]

mod cli;
mod client;
mod command;
mod config;
mod util;

pub use cli::{run, setup};
pub use command::Args;
