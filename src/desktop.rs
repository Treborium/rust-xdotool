use std::process::Output;

use crate::command::options::{SyncOption, SetDesktopOption};
use crate::command::{sub_commands, Command, OptionVec};
use crate::run;

pub fn activate_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::WindowActivate(options));
    run(c, window)
}

pub fn get_activate_window() -> Output {
    let c = Command::Desktop(sub_commands::Desktop::GetActiveWindow);
    run(c, "")
}

pub fn set_num_desktops(num: u8) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::SetNumDesktops);
    run(c, &num.to_string())
}

pub fn get_num_desktops() -> Output {
    let c = Command::Desktop(sub_commands::Desktop::GetNumDesktops);
    run(c, "")
}

pub fn set_desktop_viewport(x: u16, y: u16) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::SetDesktopViewport);
    let args = format!("{} {}", x, y);
    run(c, &args)
}

pub fn get_desktop_viewport() -> Output {
    let c = Command::Desktop(sub_commands::Desktop::GetDesktopViewport);
    run(c, "")
}

pub fn set_desktop(desktop_number: u8, options: OptionVec<SetDesktopOption>) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::SetDesktop(options));
    run(c, &desktop_number.to_string())
}

pub fn get_desktop() -> Output {
    let c = Command::Desktop(sub_commands::Desktop::GetDesktop);
    run(c, "")
}

pub fn set_desktop_for_window(window: &str, desktop_number: u8) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::SetDesktopForWindow);
    let args = format!("{} {}", window, desktop_number);
    run(c, &args)
}

pub fn get_desktop_for_window(window: &str) -> Output {
    let c = Command::Desktop(sub_commands::Desktop::GetDesktopForWindow);
    run(c, window)
}
