use std::io::{self, Read, Write};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{option, thread};

use rand::Rng;

use super::utils::structs::{
    AtkSkill, AtkType, AttackElement, BuffType, DebuffType, Effect, EffectSkill, EffectTarget, EffectType, Skill
};
use super::utils::{level::Level, obstacle::Obstacle, structs::Entity};

use super::graphical_interface::{self, clear_terminal};

fn random_range(first: i32, last: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(first..=last);
    random_number
}

//checks the colition with an obstacle/wall
fn check_colitions_obstacle(p: &Entity, os: &Vec<Obstacle>) -> bool {
    for o in os {
        let check1 = p.pos.0 == o.pos.0;
        let check2 = p.pos.0 == o.pos.0;
        let check3 = p.pos.1 == o.pos.1;
        let check4 = p.pos.1 == o.pos.1;

        if (check1 || check2) && (check3 || check4) {
            return true;
        }
    }
    false
}
//checks if there's a colition with an enemy
fn check_colitions_enemy(mut p: &Entity, mut es: &Vec<Entity>) -> bool {
    for e in es {
        let check1 = p.pos.0 == e.pos.0;
        let check2 = p.pos.0 == e.pos.0;
        let check3 = p.pos.1 == e.pos.1;
        let check4 = p.pos.1 == e.pos.1;

        if (check1 || check2) && (check3 || check4) {
            return true;
        }
    }

    false
}

//searches the enemy the player colided with
fn search_battle_enemy<'a>(mut p: &Entity, mut es: &'a Vec<Entity>) -> Option<&'a Entity> {
    for e in es {
        if check_colitions_enemy(p, &vec![e.clone()]) {
            return Some(e);
        }
    }
    return None;
}

//map movement gameplay
pub fn map_game(
    player: &mut Entity,
    obstacles: Vec<Obstacle>,
    mut enemies: Vec<Entity>,
    mut level: Level,
) {
    let mut game = true;
    let mut turn = 0;

    while game {
        graphical_interface::clear_terminal();

        println!();
        println!("{}, {}", player.pos.0, player.pos.1);
        level.update_grid(&player, &obstacles, &enemies);
        level.print_grid();
        println!();

        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get the user's input");

        match input.as_str().trim() {
            "w" => {
                if (player.pos.1 > 0) {
                    player.move_up();
                    if check_colitions_obstacle(&player, &obstacles) {
                        player.move_down();
                    }
                    if check_colitions_enemy(&player, &enemies) {
                        match battle(player, &enemies) {
                            Some(a) => (),
                            None => player.move_down(),
                        }
                    }
                }
            }
            "a" => {
                if (player.pos.0 > 0) {
                    player.move_left();
                    if check_colitions_obstacle(&player, &obstacles) {
                        player.move_right();
                    }
                    if check_colitions_enemy(&player, &enemies) {
                        match battle(player, &enemies) {
                            Some(a) => (),
                            None => player.move_right(),
                        }
                    }
                }
            }
            "s" => {
                if (player.pos.1 < level.height - 1) {
                    player.move_down();
                    if check_colitions_obstacle(&player, &obstacles) {
                        player.move_up();
                    }
                    if check_colitions_enemy(&player, &enemies) {
                        match battle(player, &enemies) {
                            Some(a) => (),
                            None => player.move_up(),
                        }
                    }
                }
            }
            "d" => {
                if (player.pos.0 < level.width - 1) {
                    player.move_right();
                    if check_colitions_obstacle(&player, &obstacles) {
                        player.move_left();
                    }
                    if check_colitions_enemy(&player, &enemies) {
                        match battle(player, &enemies) {
                            Some(a) => (),
                            None => player.move_left(),
                        }
                    }
                }
            }
            _ => println!("stupid input"),
        }
    }
}

//battle gameplay
pub fn battle(player: &mut Entity, mut enemies: &Vec<Entity>) -> Option<i32> {
    //gets the enemy to fight
    let mut fight_enemy = search_battle_enemy(&player, &enemies).expect("failed to find the enemy");
    let mut battle = true;

    while battle {
        let mut opt_string = String::new();
        let mut select_action_menu = true;

        while select_action_menu {
            graphical_interface::clear_terminal();
            graphical_interface::battle_ui(&player, fight_enemy);

            io::stdout().flush();
            io::stdin()
                .read_line(&mut opt_string)
                .expect("Failed to read user input in battle");

            let mut opt: usize = opt_string
                .trim()
                .parse()
                .expect("Failed to converst String into usize");

            match opt {
                //attack
                1 => select_action_menu = false,
                //skill
                2 => {
                    graphical_interface::clear_terminal();
                    graphical_interface::skill_menu(&player);
                    let mut skill_opt = String::from("");
                    io::stdin()
                        .read_line(&mut skill_opt)
                        .expect("there was an error reading the user's output");
                    let skill_opt_int: u8 = skill_opt
                        .trim()
                        .parse()
                        .expect("erorr transforming String into u8");

                    match skill_opt_int {
                        1 => if player.skills[0].name != "empty".to_string() {},
                        2 => (),
                        3 => (),
                        4 => (),
                        _ => println!(" wrong number "),
                    }
                }
                //defense
                3 => select_action_menu = false,
                //flee
                4 => {
                    if random_range(1, 100) > 80 {
                        select_action_menu = false;
                        battle = false;
                    } else {
                        graphical_interface::clear_terminal();
                        println!("\n\n\n          Escape failed       ");
                        sleep(Duration::from_secs(2));
                        graphical_interface::clear_terminal()
                    }
                }
                _ => println!("incorrect option try again"),
            }
            opt_string = String::new();
        }
    }

    None
}

fn use_skill(mut p: &mut Entity, mut e: &mut Entity, s: &Skill) {
    let mut effects = &mut p.effects;
    let mut enemy_effects = &mut e.effects;

    if !s.effect_skill.effects.is_empty() {
        for effect in &s.effect_skill.effects {
            match effect.effect_target {
                EffectTarget::TargetEnemy => apply_effect(e, effect),
                EffectTarget::TargetSelf => apply_effect(p, effect),
                EffectTarget::None => (),
            }
        }
    }

}

fn apply_effect(target: &mut Entity, effect: &Effect) {
    target.effects.push(effect.clone());
}

fn calc_damage(caster: &mut Entity, target: &mut Entity, skill: &Skill){
    let mut damage = 0;

    let mut mv = skill.atk_skill.motion_value.clone();
    let mut  atk = caster.atk.clone();
    let mut atk_up = 0.0;
    let mut flat_atk_up = 0;
    let mut ele_damage = 0.0;
    let mut fire_up = 0.0;
    let mut ice_up = 0.0;
    let mut lightning_up = 0.0;
    let mut physical_up = 0.0;
    let mut skill_up = 0.0;
    let mut basic_up = 0.0;

    for buffs in &caster.effects {
        match &buffs.effect_type {
            EffectType::Buff(a) =>{
                match a{
                    BuffType::AtkUp(b) => atk_up += b,
                    BuffType::FlatAtkUp(b) => flat_atk_up += b,
                    BuffType::ElementalUp(b) => ele_damage += b,
                    BuffType::FireUp(b) => fire_up += b,
                    BuffType::IceUp(b) => ice_up += b,
                    BuffType::LightningUp(b) => lightning_up+= b,
                    BuffType::PhysicalUp(b) => physical_up += b,
                    BuffType::SkillUp(b) => skill_up += b,
                    BuffType::BasicUp(b) => basic_up += b,
                    _ => (),
                }
            },
            EffectType::Debuff(a) => {
                match a{
                    DebuffType::AtkDown(b) => atk_up -= b,
                    DebuffType::FlatAtkDown(b) => flat_atk_up -= b,
                    DebuffType::ElementalDown(b)=> ele_damage -= b,
                    DebuffType::FireDown(b)=> fire_up -= b,
                    DebuffType::IceDown(b)=> ice_up -= b,
                    DebuffType::LightningDown(b)=>(),
                    DebuffType::PhysicalDown(b)=>(),
                    DebuffType::SkillDown(b)=>(),
                    DebuffType::BasicDown(b)=>(),
                    _ =>(),
                }
            },
            _ => (),
        }
    }

    match skill.atk_skill.attack_type {
        AtkType::Basic => ele_damage += basic_up,
        AtkType::Skill => ele_damage += skill_up,
        _ => (),
    }

    match skill.atk_skill.attack_element {
        AttackElement::Fire => ele_damage += fire_up,
        AttackElement::Ice => ele_damage += ice_up,
        AttackElement::Lightning => ele_damage += lightning_up,
        AttackElement::Physical => ele_damage += physical_up,
        AttackElement::None => (),
    }

    

    for effects in &target.effects{
        match &effects.effect_type {
            EffectType::Buff(a) => (),
            EffectType::Debuff(a) => (),
            _ => (),
        }
    }


}