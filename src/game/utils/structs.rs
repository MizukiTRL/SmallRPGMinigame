use std::{clone, time::Duration};

#[derive(Clone)]
pub enum State {
    Normal,
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

//skills used in combat
#[derive(Clone)]
pub struct Skill {
    name: String,
    cost: i32,
    atk_skill: AtkSkill,
    effect_skill: EffectSkill,
}

impl Skill {}

//Damage types
pub enum AttackElement {
    Fire,
    Water,
    Lightning,
    Physical,
}

//Applies damage on the skill
#[derive(Clone)]
pub struct AtkSkill {
    motion_value: f32,
}

impl AtkSkill {
    pub fn new(mv: f32) -> Self {
        AtkSkill { motion_value: mv }
    }

    pub fn new_empty() -> Self {
        AtkSkill { motion_value: 0.0 }
    }
}

//Applies effects to skils
#[derive(Clone)]
pub struct EffectSkill {
    effects: Vec<Effect>,
}
#[derive(Clone)]
pub struct Effect {
    duration: u8,
    effect_type: EffectType,
    effect_target: EffectTarget,
}

#[derive(Clone)]
enum EffectTarget {
    TargetEnemy,
    TargetSelf,
}
#[derive(Clone)]
enum EffectType {
    Buff(BuffType),
    Debuff(DebuffType),
    Heal(i32),
    None,
}

#[derive(Clone)]
enum BuffType {
    AtkUp(f32),
    FlatAtkUp(u32),
    ElementalUp(f32),
    FireUp(f32),
    IceUp(f32),
    LightningUp(f32),
    PhysicalUp(f32),
    SkillUp(f32),
    BasicUp(f32),
    DefUp(f32),
    FlatDefUp(u32),
    ElementalResUp(f32),
    FireResUp(f32),
    IceResUp(f32),
    LightningResUp(f32),
    PhysicalResUp(f32),
}

#[derive(Clone)]
enum DebuffType {
    Stun,
    Poison,
    AtkDown(f32),
    FlatAtkDown(u32),
    DefDown(f32),
    FlatDefDown(u32),
    ElementalDown(f32),
    FireDown(f32),
    IceDown(f32),
    LightningDown(f32),
    PhysicalDown(f32),
    SkillDown(f32),
    BasicDown(f32),
    ElementalResDown(f32),
    FireResDown(f32),
    IceResDown(f32),
    LightningResDown(f32),
    PhysicalResDown(f32),
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
