use std::process::Output;

use crate::command::options::{SearchOption, WindowMoveOption, WindowSizeOption};
use crate::command::{sub_commands, Command, OptionVec};
use crate::run;

pub fn search(s: &str, options: OptionVec<SearchOption>) -> Output {
    let c = Command::Window(sub_commands::Window::Search(options));
    run(c, s)
}
