use std::io::{self, Read, Write};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{option, thread};

use rand::Rng;

use super::utils::structs::{
    AtkSkill, AtkType, AttackElement, BuffType, DebuffType, Effect, EffectSkill, EffectTarget,
    EffectType, Skill, State,
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
fn search_battle_enemy<'a>(mut p: &Entity, mut es: &'a mut Vec<Entity>) -> Option<&'a mut Entity> {
    for mut e in es {
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
                        match battle(player, &mut enemies) {
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
                        match battle(player, &mut enemies) {
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
                        match battle(player, &mut enemies) {
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
                        match battle(player, &mut enemies) {
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
pub fn battle(player: &mut Entity, mut enemies: &mut Vec<Entity>) -> Option<i32> {
    //gets the enemy to fight
    let mut fight_enemy = match search_battle_enemy(&player, &mut enemies) {
        Some(a) => a,
        None => panic!("failed to find an enemy"),
    };
    let mut battle = true;

    while battle {
        let mut opt_string = String::new();
        let mut select_action_menu = true;

        match fight_enemy.state {
            State::Dead => {
                battle = false;
                select_action_menu = false;
            }
            State::Normal => (),
            _ => (),
        }
        match player.state {
            State::Dead => {
                battle = false;
                select_action_menu = false;
            }
            _ => (),
        }
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
                1 => {
                    select_action_menu = false;
                    use_skill(player, fight_enemy, player.basic.clone());
                }
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
                        1 => {
                            if player.skills[0].name != "empty".to_string() {
                                use_skill(player, fight_enemy, player.skills[0].clone());
                                println!("skill 1 used");
                                select_action_menu = false;
                            }
                        }
                        2 => {
                            if player.skills[1].name != "empty".to_string() {
                                use_skill(player, fight_enemy, player.skills[1].clone());
                                select_action_menu = false;
                            }
                        }
                        3 => {
                            if player.skills[2].name != "empty".to_string() {
                                use_skill(player, fight_enemy, player.skills[2].clone());
                                select_action_menu = false;
                            }
                        }
                        4 => {
                            if player.skills[3].name != "empty".to_string() {
                                use_skill(player, fight_enemy, player.skills[3].clone());
                                select_action_menu = false;
                            }
                        }
                        _ => println!(" wrong number "),
                    }
                }
                //defense
                3 => player.effects.push(Effect::new(
                    "defending".to_string(),
                    1,
                    EffectType::Buff(BuffType::FlatDefUp(30)),
                    EffectTarget::TargetSelf,
                    false,
                )),
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

            
            let rand_num: usize = random_range(0, 6) as usize;

            if rand_num < 6{
                if fight_enemy.skills[rand_num].name != ""{
                    use_skill(fight_enemy, player, fight_enemy.skills[rand_num].clone());
                }
            } else {
                use_skill(fight_enemy, player, fight_enemy.basic.clone());
            }
        }
    }

    None
}

fn use_skill(mut caster: &mut Entity, mut target: &mut Entity, s: Skill) {
    let mut effects = &mut caster.effects;
    let mut target_effects = &mut target.effects;

    if !s.effect_skill.effects.is_empty() {
        for effect in &s.effect_skill.effects {
            match effect.effect_target {
                EffectTarget::TargetEnemy => apply_effect(target, effect),
                EffectTarget::TargetSelf => apply_effect(caster, effect),
                EffectTarget::None => (),
            }
        }
    }

    let mut damage = 0;
    if s.atk_skill.motion_value > 0.0 {
        damage = calc_damage(caster, target, &s);
        take_damage(target, damage);
    }

    graphical_interface::damage_promp(caster, target, &s, damage)
}

fn take_damage(target: &mut Entity, damage: i32) {
    target.cur_hp -= damage;
    if target.cur_hp <= 0 {
        target.state = State::Dead;
        target.cur_hp = 0;
    }
}

fn apply_effect(target: &mut Entity, effect: &Effect) {
    let len = target.effects.len();
    for i in 0..len {
        if target.effects[i].name == effect.name {
            match effect.effect_type {
                EffectType::Heal(a) => {
                    target.cur_hp += a;
                    if target.cur_hp > target.max_hp {
                        target.cur_hp = target.max_hp;
                    }
                }
                _ => match effect.can_stack {
                    true => target.effects.push(effect.clone()),
                    false => target.effects[i].duration = effect.duration.clone(),
                },
            }
        }
    }
}

fn tick_down_effect(entity: &mut Entity) {
    let mut len = entity.effects.len();
    let mut expired_list: Vec<usize> = vec![];

    for i in 0..len {
        if entity.effects[i].duration == 0 {
            expired_list.push(i);
        } else {
            entity.effects[i].duration -= 1;
        }
    }

    for index in expired_list.iter().rev() {
        entity.effects.remove(*index);
    }
}

fn calc_damage(caster: &mut Entity, target: &mut Entity, skill: &Skill) -> i32 {
    let mut damage = 0;

    let mut mv = skill.atk_skill.motion_value.clone();
    let mut atk = caster.atk.clone();
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
            EffectType::Buff(a) => match a {
                BuffType::AtkUp(b) => atk_up += b,
                BuffType::FlatAtkUp(b) => flat_atk_up += b,
                BuffType::ElementalUp(b) => ele_damage += b,
                BuffType::FireUp(b) => fire_up += b,
                BuffType::IceUp(b) => ice_up += b,
                BuffType::LightningUp(b) => lightning_up += b,
                BuffType::PhysicalUp(b) => physical_up += b,
                BuffType::SkillUp(b) => skill_up += b,
                BuffType::BasicUp(b) => basic_up += b,
                _ => (),
            },
            EffectType::Debuff(a) => match a {
                DebuffType::AtkDown(b) => atk_up -= b,
                DebuffType::FlatAtkDown(b) => flat_atk_up -= b,
                DebuffType::ElementalDown(b) => ele_damage -= b,
                DebuffType::FireDown(b) => fire_up -= b,
                DebuffType::IceDown(b) => ice_up -= b,
                DebuffType::LightningDown(b) => lightning_up -= b,
                DebuffType::PhysicalDown(b) => physical_up -= b,
                DebuffType::SkillDown(b) => skill_up -= b,
                DebuffType::BasicDown(b) => basic_up -= b,
                _ => (),
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

    damage =
        ((((atk as f32 * (1.0 + atk_up)) * mv) + flat_atk_up as f32) * (1.0 + ele_damage)) as i32;

    //println!("{}", damage);

    let mut def = target.def.clone();
    let mut def_up = 1.0;
    let mut flat_def_up = 0;
    let mut ele_res = 1.0;
    let mut fire_res = 1.0;
    let mut ice_res = 1.0;
    let mut lightning_res = 1.0;
    let mut physical_res = 1.0;

    for effects in &target.effects {
        match &effects.effect_type {
            EffectType::Buff(a) => match a {
                BuffType::DefUp(b) => def_up += b,
                BuffType::FlatDefUp(b) => flat_def_up += b,
                BuffType::ElementalResUp(b) => ele_res *= 1.0 - b,
                BuffType::FireResUp(b) => fire_res *= 1.0 - b,
                BuffType::IceResUp(b) => ice_res *= 1.0 - b,
                BuffType::LightningResUp(b) => lightning_res *= 1.0 - b,
                BuffType::PhysicalResUp(b) => physical_res *= 1.0 - b,
                _ => (),
            },
            EffectType::Debuff(a) => match a {
                DebuffType::DefDown(b) => def_up -= b,
                DebuffType::FlatDefDown(b) => flat_def_up -= b,
                DebuffType::ElementalResDown(b) => ele_damage *= 1.0 + b,
                DebuffType::FireResDown(b) => fire_res *= 1.0 + b,
                DebuffType::IceResDown(b) => ice_res *= 1.0 + b,
                DebuffType::LightningResDown(b) => lightning_res *= 1.0 + b,
                DebuffType::PhysicalResDown(b) => physical_res *= 1.0 + b,
                _ => (),
            },
            _ => (),
        }
    }

    match skill.atk_skill.attack_element {
        AttackElement::Fire => ele_res *= fire_res,
        AttackElement::Ice => ele_res *= ice_res,
        AttackElement::Lightning => ele_res *= lightning_res,
        AttackElement::Physical => ele_res *= lightning_res,
        AttackElement::None => (),
    }

    let two: f32 = 1.005;

    // formula: (dmg/2^def) * ele

    let mut total_damage_dealt = ((damage as f32) / (two.powf(def as f32)) as f32 * ele_res) as i32;

    //println!("total damage: {}", total_damage_dealt);

    total_damage_dealt
}
