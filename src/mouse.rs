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
/// - `MouseMoveOption::Window` Specify a window to move relative to. Coordinates 0,0 are at the top left of the window you choose.
///
/// # Examples
///
/// Move mouse to position x=200 y=200:
///
/// ```
/// use xdotool::{mouse, OptionVec};
///
/// let output = mouse::move_mouse(200, 200, OptionVec::new());
/// println!("{}", output.status);
/// ```
///
/// Move mouse to the top left corner of a window:
///
/// ```
/// use xdotool::command::options;
/// use xdotool::{mouse, option_vec, OptionVec};
///
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

pub fn move_mouse_relative(x: u16, y: u16, options: OptionVec<MouseMoveRelativeOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseMoveRelative(options));
    let args = format!("{} {}", x, y);
    run(c, &args)
}

pub fn click(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::Click(options));
    run(c, &button.to_string())
}

pub fn click_down(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseDown(options));
    run(c, &button.to_string())
}

pub fn click_up(button: Button, options: OptionVec<ClickOption>) -> Output {
    let c = Command::Mouse(sub_commands::Mouse::MouseUp(options));
    run(c, &button.to_string())
}

pub fn get_mouse_location() -> Output {
    let c = Command::Mouse(sub_commands::Mouse::GetMouseLocation);
    run(c, "")
}

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
