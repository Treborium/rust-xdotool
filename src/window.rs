//! Convenience functions for the window functionality in xdotool.

use crate::command::options::{
    GetWindowGeometryOption, SearchOption, SetWindowOption, SyncOption, WindowMoveOption,
    WindowSizeOption,
};
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::run;
use std::process::Output;

// TODO: implement the behave command

/// Search for window with titles, names or classes with a regular expression pattern.
/// The output is a list of X window identifiers. 
/// The default options are `SearchOption::Name`, `SearchOption::Class` and `SearchOption::ClassName`.
/// 
/// # Options
/// 
/// - `SearchOption::Class` Match against the window class.
/// - `SearchOption::ClassName` Match against the window class name.
/// - `SearchOption::MaxDepth(i32)` Set the recursion/child search depth. Default is -1, meaning infinite. 0 means no depth, only root windows will be searched.
/// - `SearchOption::Name` Match against the window name. This is the same string that is displayed in the window title bar.
/// - `SearchOption::OnlyVisible` Show only visible windows in the results. 
/// - `SearchOption::Pid(u32)` Match windows that belong to a specific process id.
/// - `SearchOption::Screen(u8)` Only match windows on a certain desktop. The default is to search all desktops.
/// - `SearchOption::Limit(32)` Stop searching after finding N matching windows.
/// - `SearchOption::Pid(u32)` Match windows that belong to a specific process id. The default is no search limit, which is equivalent to 0.
/// - `SearchOption::All` Require that all conditions are met.
/// - `SearchOption::Any` Match windows that match any condition. This is on by default.
/// - `SearchOption::Sync` Block until there are results. 
/// 
/// # Examples
/// 
/// Search for window on desktop 2: 
/// 
/// ```
/// # use xdotool::{window, option_vec, OptionVec};
/// # use xdotool::command::options;
/// let output = window::search("Firefox", option_vec![
///     options::SearchOption::Desktop(2),
/// ]);
/// println!("{}", String::from_utf8(output.stdout)?);
/// # Ok::<(), std::string::FromUtf8Error>(())
/// ```
pub fn search(s: &str, options: OptionVec<SearchOption>) -> Output {
    let c = Command::Window(sub_commands::Window::Search(options));
    run(c, s)
}

/// Output the pid owning a given window. This requires effort from the application owning a window and my not work for all windows.
pub fn get_window_pid(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowPid);
    run(c, window)
}

/// Output the name of a given window, also known as the title.
/// This is the text displayed in the window's title bar by your window manager.
pub fn get_window_name(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowName);
    run(c, window)
}

/// Output the geometry (location and position) of a window.
/// The values include: `x`, `y`, `width`, `height` and `screen number`.
/// 
/// # Options
/// 
/// - `GetWindowGeometryOption::Shell` Output values suitable for `eval` in a shell.
pub fn get_window_geometry(window: &str, options: OptionVec<GetWindowGeometryOption>) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowGeometry(options));
    run(c, window)
}

/// Prints the window id of the currently focused window.
pub fn get_window_focus(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::GetWindowFocus);
    run(c, window)
}

/// Set the window size of the given window.
/// 
/// 
/// Percentages are valid for `width` and `height`. 
/// They are relative to the geometry of the screen the window is on.
///
/// # Options
/// 
/// - `WindowSizeOption::UseHints` Use window sizing hints to set width and height.
/// - `WindowSizeOption::Sync` Wait until the window is actually resized.
/// 
/// # Examples
/// 
/// Set a terminal to be 80x24 character:
/// 
/// ```
/// # use xdotool::{window, option_vec, OptionVec};
/// # use xdotool::command::options;
/// let output = window::set_window_size("terminal-id", "80", "24", option_vec![
///     options::WindowSizeOption::UseHints,
/// ]);
/// ```
/// 
/// Make a window full height but half width:
/// 
/// ```
/// # use xdotool::{window, option_vec, OptionVec};
/// # use xdotool::command::options;
/// let output = window::set_window_size("window-id", "50%", "100%", OptionVec::new());
/// ```
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

/// Move the window to the given position.
/// 
/// 
/// If the given x coordinate is literally 'x', then the window's current x position will be unchanged.
/// The same applies for 'y'.
/// Percentages are valid width and height.
/// They are relative to the geometry of the screen the window is on.
/// 
/// # Options
/// 
/// - `WindowMoveOption::Sync` Wait until the window is actually moved.
/// - `WindowMoveOption::Relative` Make movement relative to the current window position.
/// 
/// # Examples
/// 
/// Align window to the right of the screen: 
/// 
/// ```
/// # use xdotool::{window, option_vec, OptionVec};
/// # use xdotool::command::options;
/// let output = window::move_window("window-id", "50%", "0", OptionVec::new());
/// ```
pub fn move_window(window: &str, x: &str, y: &str, options: OptionVec<WindowMoveOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMove(options));
    let args = format!("{} {} {}", window, x, y);

    run(c, &args)
}


/// Focus a window (May be ignored by some window managers or programs).
/// 

pub fn focus_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowFocus(options));
    run(c, window)
}


/// Map a window. In X11 terminology, mapping a window means making it visible to the screen. 
/// 
/// # Options
/// 
/// - `SyncOption::Sync` Wait until the window is actually mapped.
pub fn window_map(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMap(options));
    run(c, window)
}

/// Minimize a window. In X11 terminology, this is called _iconify_
/// 
/// # Options
/// 
/// - `SyncOption::Sync` Wait until the window is actually minimized. 
pub fn minimize_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowMinimize(options));
    run(c, window)
}

/// Raise the window to the top of the stack.
/// This may not work on all window managers.
pub fn raise_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowRaise);
    run(c, window)
}

/// Reparent a window. This moves the `source_window` to be a child of `destination_window`
pub fn reparent_window(source_window: &str, destination_window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowRaise);
    let args = format!("{} {}", source_window, destination_window);
    run(c, &args)
}

/// Close a window. This action will destroy the window, but will not try to kill the client controlling it.
pub fn close_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowClose);
    run(c, window)
}

/// Kill a window. This action will destroy the window and kill the client controlling it.
pub fn kill_window(window: &str) -> Output {
    let c = Command::Window(sub_commands::Window::WindowKill);
    run(c, window)
}

/// Unmap a window, making it no longer appear on your screen. 
/// 
/// # Options
/// 
/// - `SyncOption::Sync` Wait until the window is actually unmapped.
pub fn unmap_window(window: &str, options: OptionVec<SyncOption>) -> Output {
    let c = Command::Window(sub_commands::Window::WindowUnmap(options));
    run(c, window)
}

/// Set properties about a window.
/// 
/// # Options
/// 
/// - `SetWindowOption::Name(String)` Set window WM_NAME (usually the window title).
/// - `SetWindowOption::IconName(String)` Set window WM_ICON_NAME (usually the window title when minimized).
/// - `SetWindowOption::Role(String)` Set window WM_WINDOW_ROLE.
/// - `SetWindowOption::ClassName(String)` Set window class name (not to be confused with window class).
/// - `SetWindowOption::Class(String)` Set window class.
/// - `SetWindowOption::Urgency(u8)` Set window urgency. If the value is 1 the window will be marked urgent, and the window manager will somehow highlight it for user's attention. If the value is 0, the window will be marked non-urgent.
/// - `SetWindowOption::OverrideRedirect(String)` This value is a hint to the window manager for wether or not it should be managed.
/// 
pub fn set_window(window: &str, options: OptionVec<SetWindowOption>) -> Output {
    let c = Command::Window(sub_commands::Window::SetWindow(options));
    run(c, window)
}
