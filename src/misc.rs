//! Convenience functions for miscellaneous functionality in xdotool.

use std::process::Output;

use crate::command::options::SyncOption;
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::run;

/// Execute a program. This is often useful when combined with [`mouse::behave_screen_edge`](../mouse/fn.behave_screen_edge.html) to do things like locking your screen.
/// 
/// # Options
///
/// - `SyncOption::Sync` Block until the child process exits. The child process status is then passed to the parent process which copies it.
/// 
/// # Examples

// TODO: Check if this function has the desired behaviour as described in the man page

pub fn exec(command: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Misc(sub_commands::Misc::Exec(options));
    run(c, command)
}

/// Sleep for a specified period. Fractions of seconds (like 1.3 or 0.4) are valid.
pub fn sleep(seconds: f32) -> Output {
    let c = Command::Misc(sub_commands::Misc::Sleep);
    run(c, &seconds.to_string())
}
