use std::process::Output;

use crate::command::options::SyncOption;
use crate::command::{sub_commands, Command, OptionVec};
use crate::run;

pub fn exec(command: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Misc(sub_commands::Misc::Exec(options));
    run(c, command)
}

pub fn sleep(seconds: u32) -> Output {
    let c = Command::Misc(sub_commands::Misc::Sleep);
    run(c, &seconds.to_string())
}