use std::env::args;

#[macro_use]
extern crate command_macros;

fn main() {
    let command = vec!["sudo", "apt-get", "install", "htop"];
    cmd!((command[0])[&command[1..]]).status().unwrap();
}
