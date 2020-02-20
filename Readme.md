# Rust xdotool

A wrapper for the command line tool xdotool written in Rust

## Note

While I've tried my best documenting everything as detailed as possible, please look at the man page of `xdotool` for detailed information. If a function does not behave like xdotool would, please submit an issue or even better a pull request :)

## Dependencies

[xdotool](https://github.com/jordansissel/xdotool) needs to be installed:

```shell
# Ubuntu
sudo apt-get install xdotool

# Arch Linux
sudo pacman -S xdotool

# Fedora
sudo dnf install xdotool

# You get the idea
```

## Usage

Add `xdotool` to your dependencies in your `Corgo.toml`

```toml
[dependencies]
xdotool = "0.1.0"
```

If you want the latest build use the GitHub repository as your uplink:

```toml
[dependencies]
xdotool = { git = "https://github.com/Treborium/rust-xdotool" }
```

## Examples

For examples on a certain function please view the [documentation](). 