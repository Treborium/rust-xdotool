pub enum Command {
    Keyboard(sub_commands::Keyboard),
    Mouse(sub_commands::Mouse),
    Window(sub_commands::Window),
    Desktop(sub_commands::Desktop),
    Misc(sub_commands::Misc),
}


pub mod sub_commands {
    pub enum Keyboard {
        Key(Vec<super::options::KeyboardOption>),
        KeyDown(Vec<super::options::KeyboardOption>),
        KeyUp(Vec<super::options::KeyboardOption>),
        Type(Vec<super::options::KeyboardOption>),
    }
    
    pub enum Mouse {
        MouseMove(Vec<super::options::MouseMoveOption>),
        MouseMoveRelative(Vec<super::options::MouseMoveRelativeOption>),
        Click(Vec<super::options::ClickOption>),
        MouseDown(Vec<super::options::ClickOption>),
        MouseUp(Vec<super::options::ClickOption>),
        GetMouseLocation,
        BehaveScreenEdge(Vec<super::options::BehaveScreenEdgeOption>),
    }
    
    pub enum Window {
        SearchVec(Vec<super::options::SearchOption>),
        SelectWindow,
        Behave,
        GetWindowPid,
        GetWindowName,
        GetWindowGeometry(Vec<super::options::GetWindowGeometryOption>),
        GetWindowFocus,
        WindowSize(Vec<super::options::WindowSizeOption>),
        WindowMove(Vec<super::options::WindowMoveOption>),
        WindowFocus(Vec<super::options::Sync>),
        WindowMap(Vec<super::options::Sync>),
        WindowMinimize(Vec<super::options::Sync>),
        WindowRaise,
        WindowReparent,
        WindowClose,
        WindowKill,
        WindowUnmap(Vec<super::options::Sync>),
        SetWindow(Vec<super::options::SetWindowOption>),
    }
    
    pub enum Desktop {
        WindowActivate(Vec<super::options::Sync>),
        GetActiveWindow,
        SetNumDesktops,
        GetNumDesktops,
        GetDesktopViewport,
        SetDesktopViewport,
        SetDesktop,
        GetDesktop,
        SetDesktopForWindow(Vec<super::options::SetDesktopOption>),
        GetDesktopForWindow,
    }
    
    pub enum Misc {
        Exec(Vec<super::options::Sync>),
        Sleep,
    }
}

pub mod options {
    pub enum KeyboardOption {
        Window(String),
        ClearModifiers,
        Delay(u32),
    }

    pub enum MouseMoveOption {
        Window(String),
        Screen(u8),
        Polar,
        ClearModifiers,
        Sync,
    }

    pub enum MouseMoveRelativeOption {
        Polar,
        Sync,
        ClearModifiers,
    }

    pub enum ClickOption {
        ClearModifiers,
        Repeat(u32),
        Delay(u32),
        Window(String),
    }

    pub enum BehaveScreenEdgeOption {
        Delay(u32),
        Quiesce(u32),
    }

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

    pub enum GetWindowGeometryOption {
        Shell,
    }

    pub enum WindowSizeOption {
        UseHints,
        Sync
    }

    pub enum WindowMoveOption {
        Sync,
        Relative,
    }

    pub enum Sync {
        Sync,
    }

    pub enum SetWindowOption {
        Name(String),
        IconName(String),
        Role(String),
        ClassName(String),
        Class(String),
        Urgency(u8),
        OverrideRedirect(u8),
    }

    pub enum SetDesktopOption {
        Relative,
    }
}