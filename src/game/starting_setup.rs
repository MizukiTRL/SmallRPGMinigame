use std::{env::var, io, option, thread::sleep, time::Duration, vec};

use super::{
    game::{self, battle},
    graphical_interface::{self, clear_terminal, confirm_box},
    utils::{
        level::Level,
        obstacle::Obstacle,
        structs::{AtkSkill, AttackElement, EffectSkill, Entity, EntityType, Skill},
    },
};

fn test1() {
    clear_terminal();
    let mut test_level1 = Level::new(10, 20);

    let mut p1 = Entity::new(
        String::from("test"),
        10000,
        200,
        40,
        EntityType::PlayerControlled,
        (5, 5),
        [
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
        vec![],
    );
    let o1 = vec![
        Obstacle::new((0, 4)),
        Obstacle::new((9, 9)),
        Obstacle::new((9, 5)),
    ];
    let mut e1 = vec![
        Entity::new(
            String::from("test1"),
            5000,
            100,
            20,
            EntityType::Still,
            (6, 7),
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            vec![],
        ),
        Entity::new(
            String::from("test1"),
            5000,
            100,
            20,
            EntityType::Still,
            (2, 4),
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            vec![],
        ),
    ];

    game::map_game(&mut p1, o1, e1, test_level1);
}

fn test2() {
    let mut test_level1 = Level::new(10, 10);

    let mut player = Entity::new(
        String::from("test"),
        10000,
        200,
        40,
        EntityType::PlayerControlled,
        (5, 5),
        [
            String::from("fire"),
            String::from("punch"),
            String::from("heal"),
            String::from("atkup"),
        ],
        vec![],
    );

    let mut enemies = vec![Entity::new(
        String::from("test1"),
        5000,
        100,
        20,
        EntityType::Still,
        (6, 7),
        [
            String::from("fire"),
            String::from("heal"),
            String::from(""),
            String::from(""),
        ],
        vec![],
    )];

    battle(&mut player, &enemies);
}

fn load_data() {
    todo!();
}

pub fn startup() {
    game_menu()
}

fn game_menu() {
    let mut menu1 = true;
    let mut menu2 = true;

    while menu1 {
        graphical_interface::main_menu();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("there has been an error on the input");

        let option: i32 = input
            .trim()
            .parse()
            .expect("error converting string into i32");

        match &option {
            1 => {
                clear_terminal();
                graphical_interface::confirm_box(String::from("start a new game"));
                let mut level1 = Level::new(10, 20);
                level1.print_grid();
            }
            2 => (),
            3 => (),
            4 => {
                test1();
            }
            5 => menu1 = false,
            _ => (),
        }
    }
}
