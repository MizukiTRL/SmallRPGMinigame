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
    pub skills: [Skill; 6],
    pub effects: Vec<Effect>,
    pub basic: Skill,
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
        basic: Skill
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
#[derive(Clone)]
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
#[derive(Clone)]
pub enum AttackElement {
    Fire,
    Ice,
    Lightning,
    Physical,
    None,
}
#[derive(Clone)]
pub enum AtkType{
    Basic,
    Skill,
    None,
}
//Applies damage on the skill
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Effect {
    pub name: String,
    pub duration: u8,
    pub effect_type: EffectType,
    pub effect_target: EffectTarget,
}

impl Effect {
    pub fn new(
        name: String,
        duration: u8,
        effect_type: EffectType,
        effect_target: EffectTarget,
    ) -> Self {
        Effect {
            name: name,
            duration: duration,
            effect_type: effect_type,
            effect_target: effect_target,
        }
    }
    pub fn new_empty() -> Self {
        Effect {
            name: "Empty".to_string(),
            duration: 0,
            effect_type: EffectType::None,
            effect_target: EffectTarget::None,
        }
    }
}

#[derive(Clone)]
pub enum EffectTarget {
    TargetEnemy,
    TargetSelf,
    None,
}
#[derive(Clone)]
pub enum EffectType {
    Buff(BuffType),
    Debuff(DebuffType),
    Heal(i32),
    None,
}

#[derive(Clone)]
pub enum BuffType {
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
pub enum DebuffType {
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
