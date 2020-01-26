use std::process::Command;
use std::process::Output;

pub mod command;
pub mod keyboard;
pub mod window;

pub fn run(command: command::Command, args: &str) -> Output {
    let cmd = format!("xdotool {} \"{}\"", command, args);
    println!("{}", cmd);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect(&format!("Failed to execute 'xdotool key {}", args))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
