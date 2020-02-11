use std::process::Output;

use crate::command::options::{
    GetWindowGeometryOption, SearchOption, SetWindowOption, SyncOption, WindowMoveOption,
    WindowSizeOption,
};
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::run;

pub fn search(s: &str, options: OptionVec<SearchOption>) -> Output {
    let c = Command::Window(sub_commands::Window::Search(options));
    run(c, s)
}

pub fn get_window_pid(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowPid);
    run(c, window)
}

pub fn get_window_name(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowName);
    run(c, window)
}

pub fn get_window_geometry(window: &str, options: OptionVec<GetWindowGeometryOption>) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowGeometry(options));
    run(c, window)
}

pub fn get_window_focus(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowFocus);
    run(c, window)
}

pub fn set_window_size(
    window: &str,
    width: &str,
    height: &str,
    options: OptionVec<WindowSizeOption>,
) -> Output {
    let c = Command::Window(sub_commands::Window::WindowSize(options));
    let args = format!("{} {} {}", window, width, height);

    run(c, &args)
}

pub fn move_window(window: &str, x: &str, y: &str, options: OptionVec<WindowMoveOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMove(options));
    let args = format!("{} {} {}", window, x, y);

    run(c, &args)
}

pub fn focus_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowFocus(options));
    run(c, window)
}

pub fn window_map(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMap(options));
    run(c, window)
}

pub fn minimize_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMinimize(options));
    run(c, window)
}

pub fn raise_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowRaise);
    run(c, window)
}

pub fn reparent_window(source_window: &str, destination_window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowRaise);
    let args = format!("{} {}", source_window, destination_window);
    run(c, &args)
}

pub fn close_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowClose);
    run(c, window)
}

pub fn kill_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowKill);
    run(c, window)
}

pub fn unmap_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowUnmap(options));
    run(c, window)
}

pub fn set_window(window: &str, options: OptionVec<SetWindowOption>) -> Output {
    let c = Command::Window(sub_commands::Window::SetWindow(options));
    run(c, window)
}
