use std::process::Command;
use std::process::Output;

use std::fmt;
use std::io::{self, Write};

// This function is onl visible crate internally
pub(crate) fn xdotool_key(args: &str) -> std::process::ExitStatus {
    println!("{}", format!("xdotool key {}", args));

    Command::new("sh")
        .arg("-c")
        .arg(format!("xdotool key {}", args))
        .status()
        .expect(&format!("Failed to execute 'xdotool key {}", args))
}

pub fn parse_string(s: &str) -> String {
    let len = s.chars().count();
    let mut ret = String::new();

    for (i, c) in s.chars().enumerate() {
        if i != len - 1 {
            ret.push_str(&format!("{}+", c));
        } else {
            ret.push(c);
        }
    }

    ret
}


pub fn send_keys(keys: &str) {
    let output = xdotool_key(keys);

    println!("{}", output);
}

pub fn send_keys_with_options(keys: &str, options: &[&Option]) {
    let mut args = String::new();
    for option in options {
        args = format!("{} {}", args, option);
    }

    args = format!("{} \"{}\"", args, keys);

    let output = xdotool_key(&args);
    println!("{}", output);
}

#[derive(Debug)]
pub enum Option {
    Window(String),
    ClearModifiers,
    Delay(u32),  // Delay in milliseconds
}

impl fmt::Display for Option {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Option::Window(id) => write!(f, "--window {}", id),
            Option::ClearModifiers => write!(f, "--clearmodifiers"),
            Option::Delay(ms) => write!(f, "--delay {}", ms),
        }
    }
}