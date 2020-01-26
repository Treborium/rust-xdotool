use std::process::Command;
use std::process::ExitStatus;

pub mod keyboard;
pub mod command;


pub fn run(command: command::Command, args: &str) -> ExitStatus {
    let cmd = format!("xdotool {} \"{}\"", command, args);
    println!("{}", cmd);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
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
