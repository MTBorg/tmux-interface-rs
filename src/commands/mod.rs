//! The [`commands`][`crate::commands`] module contains functions for building and executing
//! tmux commands
//!
//!
//! # Tmux Command
//!
//! ## Example
//!
//! ```text
//! tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
//! ```
//!
//! [`tmux`] - tmux binary command and it's arguments
//! [`tmux_command`] - wrapper for [`crate::cmd_builder::Cmd`] type
//! [`tmux_commands`] - wrapper for [`crate::cmd_builder::CmdList`] type
//! [`commands::NewSession`] - tmux autonomous command
//!
//!
//! bypass arguments in right order
//! NewSession::new() -> NewSession -> .build() -> TmuxCommand
//! Tmux::new() -> Tmux -> .build() -> TmuxCommand (only tmux can be passed to std::process::Command)
//!
//!
//! # Tmux Commands Variants
//!
//! Tmux commands
//! * single
//!     * autonomous
//!     * binary
//! * multiple
//!     * autonomous
//!     * binary
//!
//! # Single autonomous tmux command
//!
//! Single tmux commands can be separated in two types:
//!
//! * **autonomous tmux command**, just a command itself and it's arguments, used for invoking from
//! inside of tmux
//!
//! ## Example
//!
//! ```text
//! new-session -d -n name
//! ```
//!
//! ```
//! let new_session = NewSession::new().detached().session_name().build();
//! ```
//!
//! * **binary tmux command**, a command including tmux binary name and it's arguments used for
//! invoking from outside of tmux
//!
//! ## Example
//!
//! ```text
//! tmux -v new-session -d -n name
//! ```
//!
//! ```
//! let tmux = Tmux::new()
//!         .verbose_logging()
//!         .command(NewSession::new().detached().session_name())
//!         .build();
//! ```
//!
//! # Multiple tmux commands
//!
//! And multiple tmux commands can be combinated:
//!
//! * muliple tmux commands
//!
//! ## Example
//!
//! ```text
//! new-session -d -n name ; attach-session -t name
//! ```
//!
//! ```
//! let cmds = TmuxCommands::new()
//! ```
//!
//! * multiple tmux binary commands
//!
//! ## Example
//!
//! ```text
//! tmux new-session -d -n name ; tmux attach-session -t name
//! ```
//!
//! ```
//! let cmds = Tmux::new().verbose_logging().commands(TmuxCommands::new())
//! ```
//!
//! # See Also:
//! * [Tmux Manual -> Commands](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMANDS)
//!
pub mod common;

pub mod constants;
pub mod tmux;
pub mod tmux_command;
pub mod tmux_commands;
pub mod tmux_output;

#[cfg(test)]
pub mod tmux_command_tests;
#[cfg(test)]
pub mod tmux_commands_tests;
#[cfg(test)]
pub mod tmux_tests;

pub mod buffers;
pub mod clients_and_sessions;
pub mod global_and_session_environment;
pub mod hooks;
pub mod key_bindings;
pub mod miscellaneous;
pub mod options;
pub mod status_line;
pub mod windows_and_panes;

pub use self::tmux::Tmux;
use crate::TmuxCommand;

// XXX: ?
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html))
impl<'a> TmuxCommand<'a> {
    pub fn tmux() -> Tmux<'a> {
        Tmux::new()
    }
}
