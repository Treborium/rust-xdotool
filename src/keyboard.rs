//! Convenience functions for the keyboard functionality in xdotool.

use std::process::Output;

use crate::command::options::KeyboardOption;
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::run;

/// Type a given keystroke.
/// Generally, any valid X Keysym string will work. Multiple keys are separated by '+'.
/// Aliases exist for "alt", "ctrl", "shift", "super", and "meta" which all map to Foo_L, such as Alt_L and Control_L, etc.
/// In cases where your keyboard doesn't actually have the key you want to type, xdotool will automatically find an unused keycode and use that to type the key.
/// 
/// # Options 
/// 
/// - `KeyboardOption::Window(String)` Send keystrokes to a specific window id.
/// - `KeyboardOption::ClearModifiers` Clear modifiers before sending keystrokes.
/// - `KeyboardOption::Delay(u32)` Delay between keystrokes. Default is 12ms.
/// 
/// # Examples
/// 
/// Send the keystroke ctrl+l then Backspace as separate keystrokes with 200ms delay to the active window:
/// 
/// ```
/// # use xdotool::command::options;
/// # use xdotool::{keyboard, option_vec, OptionVec};
/// keyboard::send_key("ctrl+l BackSpace", option_vec![
///     options::KeyboardOption::Delay(200)
/// ]);
/// ```
pub fn send_key(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::Key(options));
    run(c, keys)
}

/// Same as [`send_key`}(fn.send_key.html), except only keydown (press) events are sent.
pub fn send_key_down(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::KeyDown(options));
    run(c, keys)
}

/// Same as [`send_key`}(fn.send_key.html), except only keyup (release) events are sent.
pub fn send_key_up(keys: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::KeyUp(options));
    run(c, keys)
}

/// Types as if you had typed it. Supports newlines tabs (ASCII newline and tab).
/// Each keystroke is separated by a delay given by `KeyboardOption::Delay(u32)`.
/// See [`send_key`}(fn.send_key.html) for information about possible options and examples. 
pub fn type_text(text: &str, options: OptionVec<KeyboardOption>) -> Output {
    let c = Command::Keyboard(sub_commands::Keyboard::Type(options));
    run(c, text)
}
