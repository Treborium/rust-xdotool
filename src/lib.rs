//! A simple wrapper for the `xdotool` command line tool.
//!
//! # Examples
//!
//! ```
//!
//! ```

// TODO: Add examples

use std::process::Command;
use std::process::Output;

pub mod command;
pub mod desktop;
pub mod keyboard;
pub mod misc;
pub mod mouse;
pub mod optionvec;
pub mod window;

pub use optionvec::OptionVec;

/// Execute a xdotool command.
/// This is the only function you actually need. Every other function is just for convenience.
/// However using the convenience functions is much more straight forward and therefore more desirable.
/// You should only use this function if there is no convenience function available.
///
/// # Examples
///
/// Search a window
///
/// ```
/// use std::io::Write;
///
/// use xdotool::{self, option_vec, OptionVec, window};
/// use xdotool::command::{self, options, sub_commands, Command};
///
/// let options = option_vec![options::SearchOption::Name];
/// let cmd = Command::Window(sub_commands::Window::Search(options));
/// let output = xdotool::run(cmd, "firefox");
/// std::io::stdout().write_all(&output.stdout).unwrap();
/// ```
pub fn run(command: command::Command, args: &str) -> Output {
    let cmd = format!("xdotool {} {}", command, args);
    println!("{}", cmd);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute 'xdotool key {}", args))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
