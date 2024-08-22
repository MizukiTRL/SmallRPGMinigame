use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use super::utils::structs::{BuffType, Entity, Skill};

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

pub fn battle_ui(p: &Entity, e: &Entity, points: i32) {
    println!("          Battle          ");
    println!("{}", e.name);
    println!("HP: {}/{}", e.cur_hp, e.max_hp);
    println!();
    println!();
    println!("{}", p.name);
    println!("HP: {}/{}", p.cur_hp, p.max_hp);
    println!("current points: {}", points);
    println!();
    println!("1: atack      2- skill        3-defense       4-flee");
}

pub fn skill_menu(p: &Entity, points: i32) {
    println!();
    println!("current points: ");
    println!("Select your skill:");
    let mut counter = 0;

    for s in &p.skills {
        counter += 1;
        println!("{}- {}    cost: {}", counter, s.name, s.cost);
    }
    println!("7- none");
}

pub fn damage_promp(caster: &Entity, target: &Entity, skill: &Skill, damage: i32) {
    println!();
    println!(
        "{} has used {}, dealing {} to {}",
        caster.name, skill.name, damage, target.name
    );
    sleep(Duration::from_secs(2));
}

pub fn buff_promp(caster: &Entity, target: &Entity, skill: &Skill, buff: f32, buff_type: String){
    let buff_final = (buff * 100.0) as i32;

    println!();

    println!("{} has used {}, buffing {} by {}%", caster.name, skill.name, buff_type, buff_final);

    sleep(Duration::from_secs(2));
}
