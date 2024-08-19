use std::{clone, time::Duration};

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
    atk_skill: AtkSkill,
    effect_skill: EffectSkill,

}

impl Skill {
    
}
pub enum AttackElement {
    Fire,
    Water,
    Lightning,
    Physical,
}

#[derive(Clone)]
pub struct AtkSkill {
    motion_value: f32,
}

impl AtkSkill {
    pub fn new(mv: f32) -> Self {
        AtkSkill {
            motion_value: mv,
        }
    }

    pub fn new_empty() -> Self {
        AtkSkill {
            motion_value: 0.0,
        }
    }
}
#[derive(Clone)]
pub struct  EffectSkill {
    effects: Vec<Effect>
}
#[derive(Clone)]
pub enum Effect {
    Heal(i32),
    AtkBuff(f32, u8),
    ElementalBuff(),
    Stun()
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
    pub effects: Vec<Effect>,
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
        effect: Vec<Effect>,
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
            effects: effect,
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
