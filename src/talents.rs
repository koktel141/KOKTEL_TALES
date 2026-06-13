use crate::mechanics::{Player, StatModifier};

#[derive(Debug, Clone)]
pub enum PlayerTalentTree {
    Warrior(WarriorTalents),
    Rogue(RogueTalents),
    Mage(MageTalents),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MageElement {
    Void,
    Fire,
    Ice,
    Poison,
}


#[derive(Debug, Clone)]
pub struct WarriorTalents {
    // Tank Branch
    pub iron_fortress_lvl: u32,
    pub arcane_barrier_lvl: u32,
    pub ironwill_lvl: u32,
    pub taunt_unlocked: bool,
    pub true_bulwark_unlocked: bool,

    // DPS Branch
    pub berserker_crit_lvl: u32,
    pub true_strike_unlocked: bool,
    pub war_fury_lvl: u32,
    pub executioner_lvl: u32,
    pub undying_rage_unlocked: bool,
}


#[derive(Debug, Clone)]
pub struct RogueTalents {
    // Assassin Branch
    pub quickblade_lvl: u32,
    pub armor_breach_lvl: u32,
    pub resistance_shred_lvl: u32,
    pub shadow_step_unlocked: bool,
    pub thousand_cuts_unlocked: bool,

    // Duelist Branch
    pub parry_lvl: u32,
    pub counter_attack_lvl: u32,
    pub evasion_arts_lvl: u32,
    pub blade_dance_lvl: u32,
    pub perfect_defense_unlocked: bool,
}


#[derive(Debug, Clone)]
pub struct MageTalents {
    // Healer Branch
    pub holy_shield: u32,
    pub aura_of_resilience: u32,
    pub class_synergy: u32,
    pub holy_heal: u32,
    pub resurrection: u32,
    pub heal_amplification: u32,
    pub dps_tree: Option<ElementalDpsTree>,
}
// dps mage
#[derive(Debug, Clone)]
pub struct VoidBranch {
    pub void_bolt: u32,
    pub reality_fracture: u32,
    pub void_drain: u32,
    pub null_field: u32,
    pub entropy_stacks: u32,
}

#[derive(Debug, Clone)]
pub struct FireBranch {
    pub fireball: u32,
    pub ignite: u32,
    pub fire_mastery: u32,
    pub melt_armor: u32,
    pub firestorm: u32,
}

#[derive(Debug, Clone)]
pub struct IceBranch {
    pub frostbolt: u32,
    pub deep_freeze: u32,
    pub ice_lance: u32,
    pub shatter: u32,
    pub blizzard: u32,
}

#[derive(Debug, Clone)]
pub struct PoisonBranch {
    pub venom_strike: u32,
    pub toxic_cloud: u32,
    pub weaken: u32,
    pub plague_carrier: u32,
    pub lethal_dose: u32,
}

#[derive(Debug, Clone)]
pub enum ElementalDpsTree {
    Void(VoidBranch),
    Fire(FireBranch),
    Ice(IceBranch),
    Poison(PoisonBranch),
}
impl ElementalDpsTree {
    pub fn new_void() -> Self { ElementalDpsTree::Void(VoidBranch { void_bolt: 0, reality_fracture: 0, void_drain: 0, null_field: 0, entropy_stacks: 0 }) }
    pub fn new_fire() -> Self { ElementalDpsTree::Fire(FireBranch { fireball: 0, ignite: 0, fire_mastery: 0, melt_armor: 0, firestorm: 0 }) }
    pub fn new_ice() -> Self { ElementalDpsTree::Ice(IceBranch { frostbolt: 0, deep_freeze: 0, ice_lance: 0, shatter: 0, blizzard: 0 }) }
    pub fn new_poison() -> Self { ElementalDpsTree::Poison(PoisonBranch { venom_strike: 0, toxic_cloud: 0, weaken: 0, plague_carrier: 0, lethal_dose: 0 }) }
}


impl WarriorTalents {
    pub fn new() -> Self {
        Self {
            iron_fortress_lvl: 0, arcane_barrier_lvl: 0, ironwill_lvl: 0, taunt_unlocked: false, true_bulwark_unlocked: false,
            berserker_crit_lvl: 0, true_strike_unlocked: false, war_fury_lvl: 0, executioner_lvl: 0, undying_rage_unlocked: false,
        }
    }
}

impl RogueTalents {
    pub fn new() -> Self {
        Self {
            quickblade_lvl: 0, armor_breach_lvl: 0, resistance_shred_lvl: 0, shadow_step_unlocked: false, thousand_cuts_unlocked: false,
            parry_lvl: 0, counter_attack_lvl: 0, evasion_arts_lvl: 0, blade_dance_lvl: 0, perfect_defense_unlocked: false,
        }
    }
}

impl MageTalents {
    pub fn new() -> Self {
        Self {
            holy_shield: 0,
            aura_of_resilience: 0,
            class_synergy: 0,
            holy_heal: 0,
            resurrection: 0,
            heal_amplification: 0,
            dps_tree: None,
        }
    }
}