#[derive(Clone)]
pub enum State {
    Normal,
    Poisoned,
    Stunned,
    Dead,
}
#[derive(Clone)]
pub enum EntityType {
    Still,
    Moving,
    FollowingPlayer,
    PlayerControlled,
}
#[derive(Clone)]
pub struct Skill {
    name: String,
    cost: i32,
}

impl Skill {
    pub fn new(name: String, cost: i32) -> Self {
        Skill {
            name: name,
            cost: cost,
        }
    }
    pub fn new_empty() -> Self {
        Skill {
            name: String::from(""),
            cost: 0,
        }
    }
}
pub enum AttackElement {
    Fire,
    Water,
    Lightning,
    Physical,
}

pub enum AttackType {
    Basic(),
    Skill(AttackElement),
}

pub struct AtkSkill {
    skill: Skill,
    motion_value: f32,
    attack_type: AttackType,
}

pub enum EffectType {
    Buff(),
    Debuff(),
    Heal,
    Clense,
}

pub struct Effect{
    pub name: String,
    pub effect_type: EffectType,
    pub effect_value: f32,
}

pub struct EffectSkill {
    skill: Skill,
    effect: Effect
}

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub state: State,
    pub entity_type: EntityType,
    pub pos: (i32, i32),
    pub skills: [String; 4],
}

impl Entity {
    pub fn new(
        name: String,
        max_hp: i32,
        atk: i32,
        def: i32,
        entity_type: EntityType,
        pos: (i32, i32),
        skill: [String; 4],
    ) -> Self {
        Entity {
            name: name,
            max_hp: max_hp,
            cur_hp: max_hp,
            atk: atk,
            def: def,
            state: State::Normal,
            entity_type: entity_type,
            pos: pos,
            skills: skill,
        }
    }

    pub fn move_up(&mut self) {
        self.pos.1 -= 1
    }
    pub fn move_down(&mut self) {
        self.pos.1 += 1
    }
    pub fn move_left(&mut self) {
        self.pos.0 -= 1
    }
    pub fn move_right(&mut self) {
        self.pos.0 += 1
    }
}
