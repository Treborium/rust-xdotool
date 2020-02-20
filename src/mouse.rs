//! Convenience functions for the mouse functionality in xdotool.

use std::fmt;
use std::process::Output;

use crate::command::options::{
    BehaveScreenEdgeOption, ClickOption, MouseMoveOption, MouseMoveRelativeOption,
};
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::run;

/// Move the mouse to the specific x and y coordinates on the screen
///
/// # Options
///
/// - `MouseMoveOption::Window(String)` Specify a window to move relative to. Coordinates 0,0 are at the top left of the window you choose.
/// - `MouseMoveOption::Screen(u8)` Move the mouse to the specified screen to move to.
///   This is only useful if you have multiple screens and ARE NOT using Xinerama. The default is the current screen. If you specify `MouseMoveOption::Window`, the `MouseMoveOption::Screen` option is ignored.
/// - `MouseMoveOption::Polar` Use polar coordinates. This makes `x` an angle (in degrees, 0-360, etc) and `y` the distance. Rotation starts at 'up' (0 degrees) and rotates clockwise: 90 = right, 180 = down, 270 = left. The origin defaults to the center of the current screen. If you specify a `MouseMoveOption::Window`, then the origin is the center of that window.
/// - `MouseMoveOption::Sync` After sending the mouse move request, wait until the mouse is actually moved. If no movement is necessary, we will not wait. This is useful for scripts that depend on actions being completed before moving on.
/// - `MouseMoveOption::ClearModifiers`
///
/// # Examples
///
/// Move mouse to position x=200 y=200:
///
/// ```
/// # use xdotool::{mouse, OptionVec};
/// let output = mouse::move_mouse(200, 200, OptionVec::new());
/// println!("{}", output.status);
/// ```
///
/// Move mouse to the top left corner of a window:
///
/// ```
/// # use xdotool::command::options;
/// # use xdotool::{mouse, option_vec, OptionVec};
/// let output = mouse::move_mouse(200, 200, option_vec![
///     options::MouseMoveOption::Window("window-id".to_owned())
/// ]);
/// println!("{}", output.status);
/// ```
pub fn move_mouse(x: u16, y: u16, options: OptionVec<MouseMoveOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseMove(options));
    let args = format!("{} {}", x, y);
    run(c, &args)
}

/// Move the mouse x,y pixels relative to the current position of the mouse cursor.
///
/// # Options
///
/// - `MouseMoveRelativeOption::Polar` Use polar coordinates. This makes `x` an angle (in degrees, 0-360, etc) and `y` the distance. Rotation starts at 'up' (0 degrees) and rotates clockwise: 90 = right, 180 = down, 270 = left. The origin defaults to the center of the current screen.
/// - `MouseMoveRelativeOption::Sync` After sending the mouse move request, wait until the mouse is actually moved. If no movement is necessary, we will not wait. Note that we wait until the mouse moves at all, not necessarily that it actually reaches your intended destination.
/// - `MouseMoveRelativeOption::ClearModifiers`
///
/// # Examples
///
/// Move the mouse 100 pixels to the right and 300 pixels up and wait for the action to complete:
///
/// ```
/// # use xdotool::command::options;
/// # use xdotool::{mouse, option_vec, OptionVec};
/// let output = mouse::move_mouse_relative(100, -300, option_vec![
///     options::MouseMoveRelativeOption::Sync
/// ]);
/// println!("{}", String::from_utf8(output.stdout)?);
/// # Ok::<(), std::string::FromUtf8Error>(())
/// ```
pub fn move_mouse_relative(x: i16, y: i16, options: OptionVec<MouseMoveRelativeOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseMoveRelative(options));
    let args = format!("{} {}", x, y);
    run(c, &args)
}

/// Send a click, that is, a [`click_down`](fn.click_down.html) followed by [`click_up`](fn.click_up.html) for the given button with a short delay between the two (currently 12ms).
///
/// # Options
///
/// - `ClickOption::Repeat(u32)` Specify how many times to click. Default is 1. For a double-click use `ClickOption::Repeat(2)`.
/// - `ClickOption::Delay(u32)` Specify how long, in milliseconds, to delay between clicks. This option is not used if the `ClickOption::Repeat` option is set to 1 (default).
/// - `ClickOption::Window(String)` Specify a window to send a click to.
/// - `ClickOption::ClearModifiers`
///
/// # Examples
///
/// Send a left double-click to the current mouse position:
///
/// ```
/// # use xdotool::command::options;
/// # use xdotool::mouse::{self, Button};
/// # use xdotool::{option_vec, OptionVec};
/// let output = mouse::click(Button::Left, option_vec![
///     options::ClickOption::Repeat(2)
/// ]);
/// ```
pub fn click(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::Click(options));
    run(c, &button.to_string())
}

/// Same as [`click`](fn.click.html), except only a mouse down is sent.
///
/// # Options
///
/// - `ClickOption::Repeat(u32)` Specify how many times to click. Default is 1. For a double-click use `ClickOption::Repeat(2)`.
/// - `ClickOption::Delay(u32)` Specify how long, in milliseconds, to delay between clicks. This option is not used if the `ClickOption::Repeat` option is set to 1 (default).
/// - `ClickOption::Window(String)` Specify a window to send a click to.
/// - `ClickOption::ClearModifiers`
pub fn click_down(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseDown(options));
    run(c, &button.to_string())
}

/// Same as [`click`](fn.click.html), except only a mouse up is sent.
///
/// # Options
///
/// - `ClickOption::Repeat(u32)` Specify how many times to click. Default is 1. For a double-click use `ClickOption::Repeat(2)`.
/// - `ClickOption::Delay(u32)` Specify how long, in milliseconds, to delay between clicks. This option is not used if the `ClickOption::Repeat` option is set to 1 (default).
/// - `ClickOption::Window(String)` Specify a window to send a click to.
/// - `ClickOption::ClearModifiers`
pub fn click_up(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseUp(options));
    run(c, &button.to_string())
}

/// Outputs the x, y, screen, and window id of the mouse cursor. Screen numbers will be nonzero if you have multiple monitors and are not using Xinerama.
///
/// # Examples
///
/// Print the x, y, screen and window id of the mouse cursor:
///
/// ```
/// # use xdotool::command::options;
/// # use xdotool::mouse;
/// let output = mouse::get_mouse_location();
/// println!("{}", String::from_utf8(output.stdout)?);
/// # Ok::<(), std::string::FromUtf8Error>(())
/// ```
pub fn get_mouse_location() -> Output {
    let c = Command::Mouse(sub_commands::Mouse::GetMouseLocation);
    run(c, "")
}

/// Bind an action to events when the mouse hits the screen edge or corner.
///
/// # Options
///
/// - `BehaveScreenEdgeOption::Delay(u32)` Delay in milliseconds before running the command. This allows you to require a given edge or corner to be held for a short period before your command will run. If you leave the edge or corner before the delay expires then the time will reset.
/// - `BehaveScreenEdgeOption::Quiesce(u32)`  Delay in milliseconds before the next command will run. This helps prevent accidentally running your command extra times; especially useful if you have a very short --delay (like the default of 0).
///
/// # Examples
///
/// Open firefox if mouse is in top right corner for half a second:
///
/// ```no_run
/// # use xdotool::command::{options, sub_commands, Command};
/// # use xdotool::mouse::{self, ScreenEdge};
/// # use xdotool::{option_vec, OptionVec};
/// let output = mouse::behave_screen_edge(
///     ScreenEdge::TopRight,
///     Command::Keyboard(sub_commands::Keyboard::Key(OptionVec::new())),
///     option_vec![options::BehaveScreenEdgeOption::Delay(500)]
/// );
/// ```


// TODO: Fix the command argument. Currently it only supports commands without arguments and it's not possible to pass shell commands
pub fn behave_screen_edge(
    screen_edge: ScreenEdge,
    cmd: Command,
    options: OptionVec<BehaveScreenEdgeOption>,
) -> Output {
    let bse_cmd = Command::Mouse(sub_commands::Mouse::BehaveScreenEdge(options));
    let args = format!("{} {}", screen_edge, cmd);
    run(bse_cmd, &args)
}

pub enum Button {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Button::Left => write!(f, "1"),
            Button::Middle => write!(f, "2"),
            Button::Right => write!(f, "3"),
            Button::ScrollUp => write!(f, "4"),
            Button::ScrollDown => write!(f, "5"),
        }
    }
}

pub enum ScreenEdge {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl fmt::Display for ScreenEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ScreenEdge::Left => write!(f, "left"),
            ScreenEdge::TopLeft => write!(f, "top-left"),
            ScreenEdge::Top => write!(f, "top"),
            ScreenEdge::TopRight => write!(f, "top-right"),
            ScreenEdge::Right => write!(f, "right"),
            ScreenEdge::BottomLeft => write!(f, "bottom-left"),
            ScreenEdge::Bottom => write!(f, "bottom"),
            ScreenEdge::BottomRight => write!(f, "bottom-right"),
        }
    }
}
