use std::{env::var, io, net::AddrParseError, option, string, thread::sleep, time::Duration, vec};

use super::{
    game::{self, battle},
    graphical_interface::{self, clear_terminal, confirm_box},
    utils::{
        level::Level,
        obstacle::Obstacle,
        structs::{
            AtkSkill, AtkType, AttackElement, Effect, EffectSkill, EffectTarget, EffectType,
            Entity, EntityType, Skill,
        },
    },
};

fn test1() {
    clear_terminal();
    let mut test_level1 = Level::new(10, 20);

    let mut empty_skill = Skill::new_empty();

    let mut p1 = Entity::new(
        String::from("test"),
        10000,
        2000,
        40,
        EntityType::PlayerControlled,
        (5, 5),
        [
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
        ],
        vec![],
        empty_skill.clone(),
        0.2,
        0.2,
        0.2,
        0.2,
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
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
            ],
            vec![],
            empty_skill.clone(),
            0.2,
            0.2,
            0.2,
            0.2,
        ),
        Entity::new(
            String::from("test1"),
            5000,
            100,
            20,
            EntityType::Still,
            (2, 4),
            [
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
                empty_skill.clone(),
            ],
            vec![],
            empty_skill.clone(),
            0.2,
            0.2,
            0.2,
            0.2,
        ),
    ];

    game::map_game(&mut p1, o1, e1, test_level1);
}

fn test2() {
    let mut test_level1 = Level::new(10, 10);

    let mut basic_skill = Skill::new(
        "basic".to_string(),
        0,
        AtkSkill::new(0.6, AttackElement::Physical, AtkType::Basic),
        EffectSkill::new_empty(),
    );

    let mut fire_skill = Skill::new(
        "fire".to_string(),
        2,
        AtkSkill::new(1.0, AttackElement::Fire, AtkType::Skill),
        EffectSkill::new_empty(),
    );

    let mut heal_skill = Skill::new(
        "heal".to_string(),
        3,
        AtkSkill::new_empty(),
        EffectSkill::new(vec![Effect::new(
            "heal".to_string(),
            0,
            EffectType::Heal(200),
            EffectTarget::TargetSelf,
            false,
        )]),
    );

    let mut empty_skill = Skill::new_empty();

    let mut player = Entity::new(
        String::from("test"),
        10000,
        200,
        40,
        EntityType::PlayerControlled,
        (5, 5),
        [
            fire_skill.clone(),
            heal_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
        ],
        vec![],
        basic_skill.clone(),
        0.2,
        0.2,
        0.2,
        0.2,
    );

    let mut enemies = vec![Entity::new(
        String::from("test1"),
        1000,
        100,
        20,
        EntityType::Still,
        (5, 5),
        [
            fire_skill.clone(),
            heal_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
            empty_skill.clone(),
        ],
        vec![],
        basic_skill.clone(),
        0.2,
        0.2,
        0.2,
        0.2,
    )];

    let player_json = serde_json::to_string(&player).unwrap();

    println!("{}", player_json);
    sleep(Duration::from_secs(3));

    battle(&mut player, &mut enemies);
}

enum DataType{
    Level,
    Entity,
    Obstacle,
    Skill,
    SaveFile,
}

fn load_data(data_type: DataType,) {
    todo!();
}

fn save_data(data_type: DataType, data: String){
    let address = match data_type {
        DataType::Level => (),
        DataType::Entity => (),
        DataType::Obstacle => (),
        DataType::Skill =>  (),
        DataType::SaveFile => (),
    };
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
                test2();
            }
            5 => menu1 = false,
            _ => (),
        }
    }
}
