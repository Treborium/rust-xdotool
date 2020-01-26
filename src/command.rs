use std::fmt;

#[derive(Debug)]
pub struct OptionVec<T: fmt::Display>(pub Vec<T>);

impl<T: fmt::Display> fmt::Display for OptionVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut space_separated = String::new();
        
        for thing in &self.0[0..self.0.len() - 1] {
            space_separated.push_str(&thing.to_string());
            space_separated.push_str(" ");
        }
        
        space_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", space_separated)
    }
}

#[macro_export]
macro_rules! option_vec {
    ($elem:expr; $n:expr) => (
        OptionVec(vec![$elem; $n])
    );
    ($($x:expr),*) => (
        OptionVec(<[_]>::into_vec(Box::new([$($x),*])))
    );
    ($($x:expr,)*) => (OptionVec(vec![$($x),*]))
}

pub enum Command {
    Keyboard(sub_commands::Keyboard),
    Mouse(sub_commands::Mouse),
    Window(sub_commands::Window),
    Desktop(sub_commands::Desktop),
    Misc(sub_commands::Misc),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Command::Keyboard(sub) => write!(f, "{}", sub),
            Command::Mouse(sub) => write!(f, "{}", sub),
            Command::Window(sub) => write!(f, "{}", sub),
            Command::Desktop(sub) => write!(f, "{}", sub),
            Command::Misc(sub) => write!(f, "{}", sub),
        }
    }
}


pub mod sub_commands {
    use super::OptionVec;

    #[derive(Debug)]
    pub enum Keyboard {
        Key(OptionVec<super::options::KeyboardOption>),
        KeyDown(OptionVec<super::options::KeyboardOption>),
        KeyUp(OptionVec<super::options::KeyboardOption>),
        Type(OptionVec<super::options::KeyboardOption>),
    }

    impl super::fmt::Display for Keyboard {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Keyboard::Key(o) => write!(f, "key {}", o),
                Keyboard::KeyDown(o) => write!(f, "keydown {}", o),
                Keyboard::KeyUp(o) => write!(f, "keyup {}", o),
                Keyboard::Type(o) => write!(f, "type {}", o),
            }
        }
    }
    
    #[derive(Debug)]
    pub enum Mouse {
        MouseMove(OptionVec<super::options::MouseMoveOption>),
        MouseMoveRelative(OptionVec<super::options::MouseMoveRelativeOption>),
        Click(OptionVec<super::options::ClickOption>),
        MouseDown(OptionVec<super::options::ClickOption>),
        MouseUp(OptionVec<super::options::ClickOption>),
        GetMouseLocation,
        BehaveScreenEdge(OptionVec<super::options::BehaveScreenEdgeOption>),
    }

    impl super::fmt::Display for Mouse {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Mouse::MouseMove(o) => write!(f, "mousemove {}", o),
                Mouse::MouseMoveRelative(o) => write!(f, "mousemove_relative {}", o),
                Mouse::Click(o) => write!(f, "click {}", o),
                Mouse::MouseDown(o) => write!(f, "mousedown {}", o),
                Mouse::MouseUp(o) => write!(f, "mouseup {}", o),
                Mouse::GetMouseLocation => write!(f, "getmouselocation"),
                Mouse::BehaveScreenEdge(o) => write!(f, "behave_screen_edge {}", o),
            }
        }
    }
    
    #[derive(Debug)]
    pub enum Window {
        Search(OptionVec<super::options::SearchOption>),
        SelectWindow,
        Behave,
        GetWindowPid,
        GetWindowName,
        GetWindowGeometry(OptionVec<super::options::GetWindowGeometryOption>),
        GetWindowFocus,
        WindowSize(OptionVec<super::options::WindowSizeOption>),
        WindowMove(OptionVec<super::options::WindowMoveOption>),
        WindowFocus(OptionVec<super::options::Sync>),
        WindowMap(OptionVec<super::options::Sync>),
        WindowMinimize(OptionVec<super::options::Sync>),
        WindowRaise,
        WindowReparent,
        WindowClose,
        WindowKill,
        WindowUnmap(OptionVec<super::options::Sync>),
        SetWindow(OptionVec<super::options::SetWindowOption>),
    }

    impl super::fmt::Display for Window {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Window::Search(o) => write!(f, "search {}", o),
                Window::SelectWindow => write!(f, "selectwindow"),
                Window::Behave => write!(f, "behave"),
                Window::GetWindowPid => write!(f, "getwindowpid"),
                Window::GetWindowName => write!(f, "getwindowname"),
                Window::GetWindowGeometry(o) => write!(f, "getwindowgeometry {}", o),
                Window::GetWindowFocus => write!(f, "getwindowfocus"),
                Window::WindowSize(o) => write!(f, "windowsize {}", o),
                Window::WindowMove(o) => write!(f, "windowmove {}", o),
                Window::WindowFocus(o) => write!(f, "windowfocus {}", o),
                Window::WindowMap(o) => write!(f, "windowmap {}", o),
                Window::WindowMinimize(o) => write!(f, "windowminimize {}", o),
                Window::WindowRaise => write!(f, "windowraise"),
                Window::WindowReparent => write!(f, "windowreparent"),
                Window::WindowClose => write!(f, "windowclose"),
                Window::WindowKill => write!(f, "windowkill"),
                Window::WindowUnmap(o) => write!(f, "windowunmap {}", o),
                Window::SetWindow(o) => write!(f, "set_window {}", o),
            }
        }
    }
    
    #[derive(Debug)]
    pub enum Desktop {
        WindowActivate(OptionVec<super::options::Sync>),
        GetActiveWindow,
        SetNumDesktops,
        GetNumDesktops,
        GetDesktopViewport,
        SetDesktopViewport,
        SetDesktop,
        GetDesktop,
        SetDesktopForWindow(OptionVec<super::options::SetDesktopOption>),
        GetDesktopForWindow,
    }

    impl super::fmt::Display for Desktop {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Desktop::WindowActivate(o) => write!(f, "windowactivate {}", o),
                Desktop::GetActiveWindow => write!(f, "getactivewindow"),
                Desktop::SetNumDesktops => write!(f, "set_num_desktop"),
                Desktop::GetNumDesktops => write!(f, "get_num_desktop"),
                Desktop::GetDesktopViewport => write!(f, "get_desktop_viewport"),
                Desktop::SetDesktopViewport => write!(f, "set_desktop_viewport"),
                Desktop::SetDesktop => write!(f, "set_desktop"),
                Desktop::GetDesktop => write!(f, "get_desktop"),
                Desktop::SetDesktopForWindow(o) => write!(f, "set_desktop_for_window {}", o),
                Desktop::GetDesktopForWindow => write!(f, "get_desktop_for_window"),

            }
        }
    }
    
    #[derive(Debug)]
    pub enum Misc {
        Exec(OptionVec<super::options::Sync>),
        Sleep,
    }

    impl super::fmt::Display for Misc {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Misc::Exec(o) => write!(f, "exec {}", o),
                Misc::Sleep => write!(f, "sleep"),

            }
        }
    }
}

pub mod options {
    #[derive(Debug)]
    pub enum KeyboardOption {
        Window(String),
        ClearModifiers,
        Delay(u32),
    }

    impl super::fmt::Display for KeyboardOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                KeyboardOption::Window(x) => write!(f, "--window {}", x),
                KeyboardOption::ClearModifiers => write!(f, "--clearmodifiers"),
                KeyboardOption::Delay(x) => write!(f, "--delay {}", x),
            }
        }
    }

    #[derive(Debug)]
    pub enum MouseMoveOption {
        Window(String),
        Screen(u8),
        Polar,
        ClearModifiers,
        Sync,
    }

    impl super::fmt::Display for MouseMoveOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                MouseMoveOption::Window(x) => write!(f, "--window {}", x),
                MouseMoveOption::Screen(x) => write!(f, "--screen {}", x),
                MouseMoveOption::Polar => write!(f, "--polar"),
                MouseMoveOption::ClearModifiers => write!(f, "--clearmodifiers"),
                MouseMoveOption::Sync => write!(f, "--sync"),
            }
        }
    }

    #[derive(Debug)]
    pub enum MouseMoveRelativeOption {
        Polar,
        Sync,
        ClearModifiers,
    }

    impl super::fmt::Display for MouseMoveRelativeOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                MouseMoveRelativeOption::Polar => write!(f, "--polar"),
                MouseMoveRelativeOption::ClearModifiers => write!(f, "--clearmodifiers"),
                MouseMoveRelativeOption::Sync => write!(f, "--sync"),
            }
        }
    }

    #[derive(Debug)]
    pub enum ClickOption {
        ClearModifiers,
        Repeat(u32),
        Delay(u32),
        Window(String),
    }

    impl super::fmt::Display for ClickOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                ClickOption::ClearModifiers => write!(f, "--clearmodifiers"),
                ClickOption::Repeat(x) => write!(f, "--repeat {}", x),
                ClickOption::Delay(x) => write!(f, "--delay {}", x),
                ClickOption::Window(x) => write!(f, "--window {}", x),
            }
        }
    }

    #[derive(Debug)]
    pub enum BehaveScreenEdgeOption {
        Delay(u32),
        Quiesce(u32),
    }

    impl super::fmt::Display for BehaveScreenEdgeOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                BehaveScreenEdgeOption::Delay(x) => write!(f, "--delay {}", x),
                BehaveScreenEdgeOption::Quiesce(x) => write!(f, "--quiesce {}", x),
            }
        }
    }

    #[derive(Debug)]
    pub enum SearchOption {
        Class(String),
        ClassName(String),
        MaxDepth(i32),
        Name(String),
        OnlyVisible,
        Pid(u32),
        Screen(u8),
        Desktop(u32),
        Limit(u32),
        All,
        Any,
        Sync,
    }

    impl super::fmt::Display for SearchOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                SearchOption::Class(x) => write!(f, "--class {}", x),
                SearchOption::ClassName(x) => write!(f, "--classname {}", x),
                SearchOption::MaxDepth(x) => write!(f, "--maxdepth {}", x),
                SearchOption::Name(x) => write!(f, "--name {}", x),
                SearchOption::OnlyVisible => write!(f, "--onlyvisible"),
                SearchOption::Pid(x) => write!(f, "--pid {}", x),
                SearchOption::Screen(x) => write!(f, "--screen {}", x),
                SearchOption::Desktop(x) => write!(f, "--desktop {}", x),
                SearchOption::Limit(x) => write!(f, "--limit {}", x),
                SearchOption::All => write!(f, "--all"),
                SearchOption::Any => write!(f, "--any"),
                SearchOption::Sync => write!(f, "--sync"),
            }
        }
    }

    #[derive(Debug)]
    pub enum GetWindowGeometryOption {
        Shell,
    }

    impl super::fmt::Display for GetWindowGeometryOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                GetWindowGeometryOption::Shell => write!(f, "--shell"),
            }
        }
    }

    #[derive(Debug)]
    pub enum WindowSizeOption {
        UseHints,
        Sync
    }

    impl super::fmt::Display for WindowSizeOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                WindowSizeOption::Sync => write!(f, "--sync"),
                WindowSizeOption::UseHints => write!(f, "--usehints"),
            }
        }
    }

    #[derive(Debug)]
    pub enum WindowMoveOption {
        Sync,
        Relative,
    }

    impl super::fmt::Display for WindowMoveOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                WindowMoveOption::Sync => write!(f, "--sync"),
                WindowMoveOption::Relative => write!(f, "--relative"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Sync {
        Sync,
    }

    impl super::fmt::Display for Sync {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                Sync::Sync => write!(f, "--sync"),
            }
        }
    }

    #[derive(Debug)]
    pub enum SetWindowOption {
        Name(String),
        IconName(String),
        Role(String),
        ClassName(String),
        Class(String),
        Urgency(u8),
        OverrideRedirect(u8),
    }

    impl super::fmt::Display for SetWindowOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                SetWindowOption::Name(x) => write!(f, "--name {}", x),
                SetWindowOption::IconName(x) => write!(f, "--icon-name {}", x),
                SetWindowOption::Role(x) => write!(f, "--role {}", x),
                SetWindowOption::ClassName(x) => write!(f, "--classname {}", x),
                SetWindowOption::Class(x) => write!(f, "--class {}", x),
                SetWindowOption::Urgency(x) => write!(f, "--urgency {}", x),
                SetWindowOption::OverrideRedirect(x) => write!(f, "--overrideredirect {}", x),
            }
        }
    }

    #[derive(Debug)]
    pub enum SetDesktopOption {
        Relative,
    }

    impl super::fmt::Display for SetDesktopOption {
        fn fmt(&self, f: &mut super::fmt::Formatter) -> super::fmt::Result {
            match &*self {
                SetDesktopOption::Relative => write!(f, "--relative"),
            }
        }
    }
}