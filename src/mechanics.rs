use crate::talents::PlayerTalentTree;
use crate::combat::Enemy;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum StatModifier {
    AddStrength(f64),
    AddAgility(f64),
    AddIntelligence(f64),
    AddMaxHp(f64),
    AddMaxMana(f64),
    AddArmor(f64),
    AddBlockChance(f64), // 0.05 = +5%
    AddBlockValue(f64),  // flat damage absorbed per block
    AddCritChance(f64),  // 0.10 = +10%
    AddHpRegen(f64),     // HP per turn
    AddManaRegen(f64),   // Mana per turn
    NoEffect,
}

#[derive(Debug, Clone, PartialEq,Copy)]
pub enum PlayerClass {
    Warrior,
    Rogue,
    Mage,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeaponStyle {
    TwoHanded,
    OneHandedShield,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub max_hp: f64,
    pub current_hp: f64,
    pub max_mana: f64,
    pub current_mana: f64,
    pub strength: f64,
    pub agility: f64,
    pub intelligence: f64,
    // ── Combat stats ──
    pub armor: f64,
    pub block_chance: f64,    // 0.0–1.0 (capped at 0.95 in apply)
    pub block_value: f64,     // flat dmg absorbed on block
    pub crit_chance: f64,     // 0.0–2.40 (over 1.0 converts to crit dmg later)
    pub crit_multiplier: f64, // e.g. 1.5 = 150% of base damage on crit
    pub hp_regen: f64,
    pub mana_regen: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            max_hp: 100.0, current_hp: 100.0,
            max_mana: 50.0, current_mana: 50.0,
            strength: 5.0, agility: 5.0, intelligence: 5.0,
            armor: 0.0,
            block_chance: 0.0,
            block_value: 0.0,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
            hp_regen: 1.0,
            mana_regen: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq,Copy)]
pub enum PlayerSpec {
    WarriorDPS,
    WarriorTank,
    RogueAssassin,
    RogueDuelist,
    MageHealer,
    MageElemental,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: f64,
    pub talent_points: u32,
    pub class: PlayerClass,
    pub weapon_style: WeaponStyle,
    pub stats: Stats,
    pub talents: PlayerTalentTree,
    pub gold: u32,
    pub cooldowns: HashMap<String, u32>,
    pub spec: PlayerSpec,
}

pub fn mana_cost(ability: &str) -> f64 {
    match ability {
        // no mana
        "basic_attack"    => 0.0,
        "war_fury"        => 0.0, // passive buff
        "counter_attack"  => 0.0, // reaction
        "undying_rage"    => 0.0, // threshold trigger

        // ── Warrior
        "taunt"           => 10.0,
        "executioner"     => 20.0,

        // ── Rogue 
        "shadow_step"     => 15.0,

        // ── Mage Healer
        "holy_heal"       => 30.0,
        "holy_shield"     => 35.0,
        "resurrection"    => 150.0,

        // ── Ice 
        "frostbolt"       => 20.0,
        "deep_freeze"     => 25.0,
        "ice_lance"       => 15.0,
        "blizzard"        => 70.0,

        // ── Fire 
        "fireball"        => 25.0,
        "ignite"          => 15.0,
        "firestorm"       => 70.0,

        // ── Void 
        "void_bolt"       => 18.0,
        "reality_fracture"=> 30.0,
        "void_drain"      => 22.0,
        "null_field"      => 28.0,

        // ── Poison 
        "venom_strike"    => 12.0,
        "toxic_cloud"     => 20.0,
        "weaken"          => 18.0,

        _ => 0.0,
    }
}
impl Player {
    pub fn update_derived_stats(&mut self) {
        let armor_scale = match self.spec {
        PlayerSpec::WarriorTank   => 0.65,
        PlayerSpec::RogueDuelist  => 0.30,
        _                         => 0.16,
        };
        self.stats.armor = self.stats.agility * armor_scale;
        let base_hp = match self.class {
        PlayerClass::Warrior => 150.0,
        PlayerClass::Rogue   => 100.0,
        PlayerClass::Mage    =>  80.0,
        };
        let base_mana = match self.class {
        PlayerClass::Warrior =>  50.0,
        PlayerClass::Rogue   =>  70.0,
        PlayerClass::Mage    => 150.0,
        };
        
        self.stats.max_hp = base_hp + (self.stats.strength * 10.0);
        self.stats.hp_regen = 2.0 + (self.stats.strength * 0.15);
        
        self.stats.max_mana = base_mana + (self.stats.intelligence * 12.0);
        self.stats.mana_regen = 2.0 + (self.stats.intelligence * 0.1);
        
        
        self.stats.armor = 0.1 + (self.stats.agility * 0.16);
        self.stats.current_hp   = self.stats.current_hp.min(self.stats.max_hp);
        self.stats.current_mana = self.stats.current_mana.min(self.stats.max_mana);
        
    }
    pub fn calculate_hit_damage(&self, base_dmg: f64) -> (f64, bool) {
        let mut rng = rand::thread_rng();
        let is_crit = rng.gen_bool(self.stats.crit_chance.min(1.0));
        let damage = if is_crit {
            base_dmg * self.stats.crit_multiplier
        } else {
            base_dmg
        };
        
        (damage, is_crit)
    }
    pub fn defeat_enemy(&mut self, enemy: &Enemy) {
        if enemy.is_dead() {
            println!("\n🏆 {} has been defeated!", enemy.name);
            println!("💰 Gained +{} Gold.", enemy.gold_reward);
            self.gold += enemy.gold_reward;
            
            
            self.gain_exp(enemy.exp_reward);
        }
    }
    
    pub fn use_ability(&mut self, ability_name: &str, enemy: &mut Enemy) {
// 1️⃣ Available check
    let available = self.get_available_abilities();
    if !available.contains(&ability_name.to_string()) {
        println!("❌ {} cannot use '{}'!", self.name, ability_name);
        return;
    }

    // 2️⃣ Cooldown check
    if let Some(&cd) = self.cooldowns.get(ability_name) {
        if cd > 0 {
            println!("⏳ '{}' on cooldown! {} turn(s) left.", ability_name, cd);
            return;
        }
    }

    // 3️⃣ Mana check
    let cost = mana_cost(ability_name);
    if self.stats.current_mana < cost {
        println!("💧 Not enough mana! '{}' costs {:.0} (have {:.0}/{:.0})",
            ability_name, cost, self.stats.current_mana, self.stats.max_mana);
        return;
    }
    if cost > 0.0 {
        self.stats.current_mana -= cost;
        println!("💧 -{:.0} mana  ({:.0}/{:.0})",
            cost, self.stats.current_mana, self.stats.max_mana);
    }

    match ability_name {
        "basic_attack" => {
            let mut rng = rand::thread_rng();

            if let PlayerTalentTree::Rogue(ref rogue) = self.talents {
                if rogue.flurry_blades_lvl > 0 {
                let hits = rng.gen_range(3..=5);
                println!("🌪️ FLURRY OF BLADES! {} strikes {} times!", self.name, hits);
            
                for _ in 0..hits {
                    let (dmg, is_crit) = self.calculate_hit_damage(self.base_damage() * 0.6);
                    if is_crit { println!("💥 CRITICAL HIT!"); }
                    enemy.take_damage(dmg);
            }
            return; 
        }
    }

    
    let (dmg, is_crit) = self.calculate_hit_damage(self.base_damage());
    if is_crit { println!("💥 CRITICAL HIT!"); }
    println!("⚔️ {} uses Basic Attack!", self.name);
    enemy.take_damage(dmg);

    
    let extra_chance = (self.stats.agility * 1.5) as u32;
    if rng.gen_range(1..=100) <= extra_chance {
        println!("⚡ Quick strike! Extra attack!");
        let (dmg_extra, is_crit_extra) = self.calculate_hit_damage(self.base_damage() * 0.5);
        if is_crit_extra { println!("💥 CRITICAL HIT!"); }
        enemy.take_damage(dmg_extra);
    }
    },
        "frostbolt" => {
            
            let spell_power = self.stats.intelligence * 2.5;
            let damage = {
                if let PlayerTalentTree::Mage(ref mage) = self.talents {
                    if let Some(crate::talents::ElementalDpsTree::Ice(ref ice)) = mage.dps_tree {
                        spell_power * (1.0 + (ice.frostbolt as f64 * 0.15))
                    } else { 0.0 }
                } else { 0.0 }
            };
            if damage > 0.0 {
                println!("❄️ {} casts Frostbolt!", self.name);
                enemy.take_damage(damage);
                enemy.is_frozen = true;
                println!("🥶 {} is Frozen!", enemy.name);
            }
            
            self.cooldowns.insert("frostbolt".to_string(), 1);
        }

        "ice_lance" => {
            let damage = {
                if let PlayerTalentTree::Mage(ref mage) = self.talents {
                    if let Some(crate::talents::ElementalDpsTree::Ice(ref ice)) = mage.dps_tree {
                        let base = self.stats.intelligence * 2.5
                            * (1.2 + (ice.ice_lance as f64 * 0.20));
                        if enemy.is_frozen {
                            println!("🧊 Shatter Combo! Double damage!");
                            base * 2.0
                        } else { base }
                    } else { 0.0 }
                } else { 0.0 }
            };
            if damage > 0.0 {
                println!("❄️ {} launches Ice Lance!", self.name);
                enemy.take_damage(damage);
                enemy.is_frozen = false;
            }
            self.cooldowns.insert("ice_lance".to_string(), 1);
        }

        _ => println!("❌ Ability '{}' not implemented yet!", ability_name),
    }
    }
    pub fn tick_cooldowns(&mut self) {
        for (ability, turns) in self.cooldowns.iter_mut() {
            if *turns > 0 {
                *turns -= 1;
                if *turns == 0 {
                    println!("✨ Ability '{}' is ready to use again!", ability);
                }
            }
        }
    }
    pub fn get_available_abilities(&self) -> Vec<String> {
        let mut abilities = Vec::new();

        
        abilities.push("basic_attack".to_string());

        
        match &self.talents {
            PlayerTalentTree::Warrior(warrior) => {
                
                if warrior.taunt_unlocked {
                    abilities.push("taunt".to_string());
                }
                
                if warrior.war_fury_lvl > 0 {
                    abilities.push("war_fury".to_string());
                }
                if warrior.executioner_lvl > 0 {
                    abilities.push("executioner".to_string());
                }
            }
            PlayerTalentTree::Rogue(rogue) => {
                if rogue.shadow_step_unlocked {
                    abilities.push("shadow_step".to_string());
                }
                if rogue.counter_attack_lvl > 0 {
                    abilities.push("counter_attack".to_string());
                }
            }
            PlayerTalentTree::Mage(mage) => {
                
                if mage.holy_heal > 0 {
                    abilities.push("holy_heal".to_string());
                }
                
                
                if let Some(ref dps_tree) = mage.dps_tree {
                    match dps_tree {
                        crate::talents::ElementalDpsTree::Ice(ice) => {
                            if ice.frostbolt > 0 { abilities.push("frostbolt".to_string()); }
                            if ice.deep_freeze > 0 { abilities.push("deep_freeze".to_string()); }
                            if ice.ice_lance > 0 { abilities.push("ice_lance".to_string()); }
                        }
                        crate::talents::ElementalDpsTree::Fire(fire) => {
                            if fire.fireball > 0 { abilities.push("fireball".to_string()); }
                            if fire.ignite > 0 { abilities.push("ignite".to_string()); }
                        }
                        crate::talents::ElementalDpsTree::Void(void) => {
                            if void.void_bolt > 0 { abilities.push("void_bolt".to_string()); }
                            if void.void_drain > 0 { abilities.push("void_drain".to_string()); }
                        }
                        crate::talents::ElementalDpsTree::Poison(poison) => {
                            if poison.venom_strike > 0 { abilities.push("venom_strike".to_string()); }
                            if poison.toxic_cloud > 0 { abilities.push("toxic_cloud".to_string()); }
                        }
                    }
                }
            }
        }

        abilities
    }

pub fn cast_mage_spell(&mut self, spell_name: &str, enemy: &mut Enemy) {
        
        if let PlayerTalentTree::Mage(ref mage_talents) = self.talents {
            
            
            let spell_power = self.stats.intelligence * 2.5;

            
            if let Some(ref dps_tree) = mage_talents.dps_tree {
                match dps_tree {
                    
                    //ice
                    crate::talents::ElementalDpsTree::Ice(ice) => {
                        match spell_name {
                            "frostbolt" => {
                                if ice.frostbolt == 0 { println!("❌ Spell not unlocked yet!"); return; }
                                
                                let damage = spell_power * (1.0 + (ice.frostbolt as f64 * 0.15));
                                println!("❄️ {} casts Frostbolt!", self.name);
                                enemy.take_damage(damage);
                                
                                enemy.is_frozen = true; 
                                println!("🥶 {} is now Frozen!", enemy.name);
                            }
                            "ice_lance" => {
                                if ice.ice_lance == 0 { println!("❌ Spell not unlocked yet!"); return; }
                                let mut damage = spell_power * (1.2 + (ice.ice_lance as f64 * 0.20));
                                
                                if enemy.is_frozen {
                                    damage *= 2.0;
                                    println!("🧊 Shatter Combo! Double damage on frozen target!");
                                }
                                println!("❄️ {} launches Ice Lance!", self.name);
                                enemy.take_damage(damage);
                            }
                            _ => println!("❌ Unknown Ice spell or not in this branch!"),
                        }
                    }

                    //fire
                    crate::talents::ElementalDpsTree::Fire(fire) => {
                        match spell_name {
                            "fireball" => {
                                if fire.fireball == 0 { println!("❌ Spell not unlocked yet!"); return; }
                                let damage = spell_power * (1.5 + (fire.fireball as f64 * 0.25));
                                println!("🔥 {} unleashes a massive Fireball!", self.name);
                                enemy.take_damage(damage);
                                enemy.ignite_turns = 3; 
                            }
                            _ => println!("❌ Unknown Fire spell or not in this branch!"),
                        }
                    }

                    //void & pison
                    crate::talents::ElementalDpsTree::Void(_) => println!("🔮 Void combat mechanics not implemented yet!"),
                    crate::talents::ElementalDpsTree::Poison(_) => println!("🧪 Poison combat mechanics not implemented yet!"),
                }
            } else {
                println!("❌ You don't have an Elemental DPS Spec chosen to cast these spells!");
            }
        } else {
            println!("❌ Only Mages can cast these elemental spells!");
        }
    }

    pub fn choose_elemental_spec(&mut self, element: &str) {
        if let crate::talents::PlayerTalentTree::Mage(ref mut mage_talents) = self.talents {
            match element.to_lowercase().as_str() {
                "void"   => mage_talents.dps_tree = Some(crate::talents::ElementalDpsTree::new_void()),
                "fire"   => mage_talents.dps_tree = Some(crate::talents::ElementalDpsTree::new_fire()),
                "ice"    => mage_talents.dps_tree = Some(crate::talents::ElementalDpsTree::new_ice()),
                "poison" => mage_talents.dps_tree = Some(crate::talents::ElementalDpsTree::new_poison()),
                _ => println!("❌ Invalid element type!"),
            }
            println!("🔮 {} has chosen the specialization: {}!", self.name, element);
        } else {
            println!("❌ Only Mages can choose an elemental specialization!");
        }
    }

    pub fn upgrade_mage_dps_talent(&mut self, talent_name: &str) {
        if self.talent_points == 0 {
            println!("❌ No talent points available!");
            return;
        }
        if let crate::talents::PlayerTalentTree::Mage(ref mut mage_talents) = self.talents {
            if let Some(ref mut dps_tree) = mage_talents.dps_tree {
                match dps_tree {
                    crate::talents::ElementalDpsTree::Ice(ice) => {
                        match talent_name {
                            "frostbolt" => { ice.frostbolt += 1; self.talent_points -= 1; }
                            "deep_freeze" => { ice.deep_freeze += 1; self.talent_points -= 1; }
                            "ice_lance" => { ice.ice_lance += 1; self.talent_points -= 1; }
                            _ => println!("❌ This talent does not exist in the Ice tree!"),
                        }
                    }
                    crate::talents::ElementalDpsTree::Fire(fire) => {
                        match talent_name {
                            "fireball" => { fire.fireball += 1; self.talent_points -= 1; }
                            "ignite" => { fire.ignite += 1; self.talent_points -= 1; }
                            _ => println!("❌ This talent does not exist in the Fire tree!"),
                        }
                    }
                    crate::talents::ElementalDpsTree::Void(void) => {
                        match talent_name {
                            "void_bolt" => { void.void_bolt += 1; self.talent_points -= 1; }
                            "void_drain" => { void.void_drain += 1; self.talent_points -= 1; }
                            _ => println!("❌ This talent does not exist in the Void tree!"),
                        }
                    }
                    crate::talents::ElementalDpsTree::Poison(poison) => {
                        match talent_name {
                            "venom_strike" => { poison.venom_strike += 1; self.talent_points -= 1; }
                            "toxic_cloud" => { poison.toxic_cloud += 1; self.talent_points -= 1; }
                            _ => println!("❌ This talent does not exist in the Poison tree!"),
                        }
                    }
                }
            } else {
                println!("❌ You must choose an Elemental Specialization first using choose_elemental_spec()!");
            }
        }
    }
    pub fn new(name: &str, class: PlayerClass,spec: PlayerSpec) -> Self {

    let (str, agil, intel, armor_scale) = match spec {
        PlayerSpec::WarriorDPS    => (30.0, 18.0, 15.0, 0.16),
        PlayerSpec::WarriorTank   => (32.0, 15.0, 15.0, 0.65),
        PlayerSpec::RogueAssassin => (20.0, 35.0, 10.0, 0.16),
        PlayerSpec::RogueDuelist  => (18.0, 30.0, 12.0, 0.30),
        PlayerSpec::MageHealer    => ( 15.0, 8.0, 40.0, 0.16),
        PlayerSpec::MageElemental => ( 10.0,  8.0, 45.0, 0.16),
    };

    let base_hp = match class {
        PlayerClass::Warrior => 150.0,
        PlayerClass::Rogue   => 100.0,
        PlayerClass::Mage    =>  80.0,
    };
    let base_mana = match class {
        PlayerClass::Warrior =>  50.0,
        PlayerClass::Rogue   =>  70.0,
        PlayerClass::Mage    => 150.0,
    };

    let mut stats = Stats {
        max_hp: base_hp, current_hp: base_hp,
        max_mana: base_mana, current_mana: base_mana,
        strength: str, agility: agil, intelligence: intel,
        armor: agil * armor_scale, 
        crit_chance: match &class {
            PlayerClass::Warrior => 0.30,
            _                    => 0.05,
        },
        crit_multiplier: 1.5,
        hp_regen: match &class {
            PlayerClass::Warrior => 3.0,
            PlayerClass::Rogue   => 1.5,
            PlayerClass::Mage    => 1.0,
        },
        mana_regen: match &class {
            PlayerClass::Mage  => 5.0,
            PlayerClass::Rogue => 1.5,
            _                  => 1.0,
        },
        ..Default::default()
    };
        let talents = match &class {
            PlayerClass::Warrior => PlayerTalentTree::Warrior(crate::talents::WarriorTalents::new()),
            PlayerClass::Rogue => PlayerTalentTree::Rogue(crate::talents::RogueTalents::new()),
            PlayerClass::Mage => PlayerTalentTree::Mage(crate::talents::MageTalents::new()),
        };
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0.0,
            talent_points: 0,
            class,
            weapon_style: WeaponStyle::TwoHanded,
            stats,
            talents,
            gold: 0,
            spec,
            cooldowns: HashMap::new(),
        }
        
    }


    pub fn apply_modifier(&mut self, modifier: &StatModifier) {
        match modifier {
            StatModifier::AddStrength(v)     => self.stats.strength += *v,
            StatModifier::AddAgility(v)      => self.stats.agility += *v,
            StatModifier::AddIntelligence(v) => self.stats.intelligence += *v,
            StatModifier::AddMaxHp(v)        => {
                self.stats.max_hp += *v;
                self.stats.current_hp += *v; // heal for the bonus HP
            }
            StatModifier::AddMaxMana(v)      => {
                self.stats.max_mana += *v;
                self.stats.current_mana += *v;
            }
            StatModifier::AddArmor(v)        => self.stats.armor += *v,
            StatModifier::AddBlockChance(v)  => {
                self.stats.block_chance = (self.stats.block_chance + *v).min(0.95)
            }
            StatModifier::AddBlockValue(v)   => self.stats.block_value += *v,
            StatModifier::AddCritChance(v)   => {
                self.stats.crit_chance = (self.stats.crit_chance + *v).min(2.40)
            }
            StatModifier::AddHpRegen(v)      => self.stats.hp_regen += *v,
            StatModifier::AddManaRegen(v)    => self.stats.mana_regen += *v,
            StatModifier::NoEffect           => {}
        }
        self.update_derived_stats(); 
    }

    pub fn set_weapon_style(&mut self, style: WeaponStyle) {
        
        if self.weapon_style == WeaponStyle::OneHandedShield {
            self.stats.block_chance = 0.0;
            self.stats.block_value = 0.0;
        }
        
        if style == WeaponStyle::OneHandedShield {
            self.stats.block_chance = 0.15; // base 15% block
            self.stats.block_value = 8.0;   // wood shield default
        }
        self.weapon_style = style;
    }

    /// Raw damage output before enemy armor reduction.
    pub fn base_damage(&self) -> f64 {
        let class_scaling_bonus = match self.class {
            PlayerClass::Warrior => self.stats.strength,      
            PlayerClass::Rogue => self.stats.agility,         
            PlayerClass::Mage => self.stats.intelligence,     
        };
        let base = 10.0 + class_scaling_bonus;
        match self.weapon_style {
        WeaponStyle::TwoHanded       => base * 1.20,
        WeaponStyle::OneHandedShield => base * 0.75,
    }
    }
    

    pub fn exp_to_next_level(&self) -> f64 {
        100.0 * 1.4_f64.powi((self.level - 1) as i32)
    }

    pub fn gain_exp(&mut self, amount: f64) {
        println!("✨ Gained +{:.0} EXP.", amount);
        self.exp += amount;
        loop {
            let needed = self.exp_to_next_level();
            if self.exp < needed { break; }
            self.exp -= needed;
            self.level_up();
        }
    }
    pub fn level_up(&mut self) {
        self.level += 1;
        self.talent_points += 1;


        match self.class {
            PlayerClass::Warrior => {
                self.stats.strength += 3.0;
                self.stats.agility += 1.0;
                self.stats.intelligence += 1.0;
            },
            PlayerClass::Rogue => {
                self.stats.strength += 1.0;
                self.stats.agility += 3.0;
                self.stats.intelligence += 1.0;
            },
            PlayerClass::Mage => {
                self.stats.strength += 1.0;
                self.stats.agility += 1.0;
                self.stats.intelligence += 3.0;
            },
    }

    self.update_derived_stats();
    
    self.stats.current_hp = self.stats.max_hp;
    self.stats.current_mana = self.stats.max_mana;
        println!("\n🎉🎉🎉 LEVEL UP! You are now Level {}! 🎉🎉🎉", self.level);
        println!("⭐ +1 Talent Point Available! (Total: {})", self.talent_points);
        println!("❤️ HP and Mana fully restored and increased!");
    }



    /// Call at the end of each combat turn.
    pub fn regen_per_turn(&mut self) {
        self.stats.current_hp = (self.stats.current_hp + self.stats.hp_regen)
            .min(self.stats.max_hp);
        self.stats.current_mana = (self.stats.current_mana + self.stats.mana_regen)
            .min(self.stats.max_mana);
    }

    pub fn print_stats(&self) {
        let weapon = match self.weapon_style {
            WeaponStyle::TwoHanded       => "Two-Handed",
            WeaponStyle::OneHandedShield => "1H + Shield",
        };
        println!(
            "╔═ {} | {:?} | Lv{} | EXP {:.0}/{:.0} | TP: {} ═╗",
            self.name, self.class, self.level,
            self.exp, self.exp_to_next_level(),
            self.talent_points
        );
        println!(
            "  HP     {:.0}/{:.0}   Mana {:.0}/{:.0}",
            self.stats.current_hp, self.stats.max_hp,
            self.stats.current_mana, self.stats.max_mana
        );
        println!(
            "  STR {:>4.0}   AGI {:>4.0}   INT {:>4.0}",
            self.stats.strength, self.stats.agility, self.stats.intelligence
        );
        println!(
            "  Armor {:>3.0}   Block {:.0}%({:.0} val)   Crit {:.0}%   Weapon: {}",
            self.stats.armor,
            self.stats.block_chance * 100.0,
            self.stats.block_value,
            self.stats.crit_chance * 100.0,
            weapon
        );
        println!(
            "  Regen  HP +{:.1}/turn   Mana +{:.1}/turn",
            self.stats.hp_regen, self.stats.mana_regen
        );
        println!("╚══════════════════════════════════════════╝");
    }
}
