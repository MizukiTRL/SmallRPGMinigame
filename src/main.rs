#![allow(unused)]

mod game;

use game::starting_setup;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!ç");

    starting_setup::startup();
}
