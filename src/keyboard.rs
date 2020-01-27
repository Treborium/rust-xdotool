use std::process::Output;

use crate::command::options::KeyboardOption;
use crate::command::{sub_commands, Command, OptionVec};
use crate::run;

pub fn send_key(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::Key(options));
    run(c, keys)
}

pub fn send_key_down(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::KeyDown(options));
    run(c, keys)
}

pub fn send_key_up(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::KeyUp(options));
    run(c, keys)
}
