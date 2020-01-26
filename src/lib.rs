use std::process::Command;
use std::process::ExitStatus;

pub mod keyboard;
pub mod command;


pub fn run(command: command::Command, args: &str) -> ExitStatus {
    Command::new("sh")
        .arg("-c")
        .arg(format!("xdotool {} {}", command, args))
        .status()
        .expect(&format!("Failed to execute 'xdotool key {}", args))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
