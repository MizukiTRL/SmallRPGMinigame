use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use super::utils::structs::Entity;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}

pub fn main_menu() {
    println!("Main Menu:");
    println!("1- Start new game");
    println!("2- load game");
    println!("3- delete save");
    println!("4- test");
    println!("5- exit game");
}

pub fn confirm_box(s: String) {
    println!();
    println!("are you sure you want to {s}?");
    println!("      1- yes           2-no");
    println!();
}

pub fn battle_ui(p: &Entity, e: &Entity) {
    println!("          Battle          ");
    println!("{}", e.name);
    println!("HP: {}/{}", e.cur_hp, e.max_hp);
    println!();
    println!();
    println!("{}", p.name);
    println!("HP: {}/{}", p.cur_hp, p.max_hp);
    println!();
    println!();
    println!("1: atack      2- skill        3-defense       4-flee");
}

pub fn skill_menu(p: &Entity) {
    println!();
    println!();
    println!("Select your skill:");
    let mut counter = 0;

    for s in &p.skills {
        counter += 1;
        println!("{}- {}", counter, s);
    }
}
