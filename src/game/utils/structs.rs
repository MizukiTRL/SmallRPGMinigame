use std::{clone, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum State {
    Normal,
    Stunned,
    Dead,
}
#[derive(Clone,Serialize, Deserialize, Debug)]
pub enum EntityType {
    Still,
    Moving,
    FollowingPlayer,
    PlayerControlled,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Entity {
    pub name: String,
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub state: State,
    pub entity_type: EntityType,
    pub pos: (i32, i32),
    pub skills: [Skill; 6],
    pub effects: Vec<Effect>,
    pub basic: Skill,
    pub fire_res: f32,
    pub ice_res: f32,
    pub lightning_res: f32,
    pub physical_res: f32,
}

impl Entity {
    pub fn new(
        name: String,
        max_hp: i32,
        atk: i32,
        def: i32,
        entity_type: EntityType,
        pos: (i32, i32),
        skill: [Skill; 6],
        effect: Vec<Effect>,
        basic: Skill,
        fire_res: f32,
        ice_res: f32,
        lightning_res: f32,
        physical_res: f32,
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
            basic: basic,
            fire_res: fire_res,
            ice_res: ice_res,
            lightning_res: lightning_res,
            physical_res: physical_res,
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

//skills used in combat
#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub cost: i32,
    pub atk_skill: AtkSkill,
    pub effect_skill: EffectSkill,
}

impl Skill {
    pub fn new(name: String, cost: i32, atk_skill: AtkSkill, effect_skill: EffectSkill) -> Self {
        Skill {
            name: name,
            cost: cost,
            atk_skill: atk_skill,
            effect_skill: effect_skill,
        }
    }
    pub fn new_empty() -> Self {
        Skill {
            name: "".to_string(),
            cost: 0,
            atk_skill: AtkSkill::new_empty(),
            effect_skill: EffectSkill::new_empty(),
        }
    }
}

//Damage types
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AttackElement {
    Fire,
    Ice,
    Lightning,
    Physical,
    None,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AtkType {
    Basic,
    Skill,
    None,
}
//Applies damage on the skill
#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct AtkSkill {
    pub motion_value: f32,
    pub attack_element: AttackElement,
    pub attack_type: AtkType,
}

impl AtkSkill {
    pub fn new(mv: f32, attack_element: AttackElement, attack_type: AtkType) -> Self {
        AtkSkill {
            motion_value: mv,
            attack_element: attack_element,
            attack_type: attack_type,
        }
    }

    pub fn new_empty() -> Self {
        AtkSkill {
            motion_value: 0.0,
            attack_element: AttackElement::None,
            attack_type: AtkType::None,
        }
    }
}

//Applies effects to skils
#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct EffectSkill {
    pub effects: Vec<Effect>,
}

impl EffectSkill {
    pub fn new(effects: Vec<Effect>) -> Self {
        EffectSkill { effects: effects }
    }

    pub fn new_empty() -> Self {
        EffectSkill { effects: vec![] }
    }
}

//Effects
#[derive(Clone, PartialEq,Serialize, Deserialize, Debug)]
pub struct Effect {
    pub name: String,
    pub duration: u8,
    pub effect_type: EffectType,
    pub effect_target: EffectTarget,
    pub can_stack: bool,
}

impl Effect {
    pub fn new(
        name: String,
        duration: u8,
        effect_type: EffectType,
        effect_target: EffectTarget,
        can_stack: bool,
    ) -> Self {
        Effect {
            name: name,
            duration: duration,
            effect_type: effect_type,
            effect_target: effect_target,
            can_stack: can_stack,
        }
    }
    pub fn new_empty() -> Self {
        Effect {
            name: "Empty".to_string(),
            duration: 0,
            effect_type: EffectType::None,
            effect_target: EffectTarget::None,
            can_stack: true,
        }
    }
}

#[derive(Clone, PartialEq,Serialize, Deserialize, Debug)]
pub enum EffectTarget {
    TargetEnemy,
    TargetSelf,
    None,
}
#[derive(Clone, PartialEq,Serialize, Deserialize, Debug)]
pub enum EffectType {
    Buff(BuffType),
    Debuff(DebuffType),
    Heal(i32),
    None,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum BuffType {
    AtkUp(f32),
    FlatAtkUp(i32),
    ElementalUp(f32),
    FireUp(f32),
    IceUp(f32),
    LightningUp(f32),
    PhysicalUp(f32),
    SkillUp(f32),
    BasicUp(f32),
    DefUp(f32),
    FlatDefUp(i32),
    ElementalResUp(f32),
    FireResUp(f32),
    IceResUp(f32),
    LightningResUp(f32),
    PhysicalResUp(f32),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum DebuffType {
    Stun,
    Poison,
    AtkDown(f32),
    FlatAtkDown(i32),
    DefDown(f32),
    FlatDefDown(i32),
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
