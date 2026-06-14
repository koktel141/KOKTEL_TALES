mod mechanics;
mod talents;
mod combat;

use mechanics::{Player, PlayerClass, PlayerSpec};
use combat::Enemy;
use talents::PlayerTalentTree;

use std::io::{self, Write};

pub fn interactive_combat(player: &mut Player, enemy: &mut Enemy) {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║ ⚔️  COMBAT INITIATED: {} vs {} ", player.name, enemy.name);
    println!("╚══════════════════════════════════════════╝\n");
    // Unlock talents for the player
    if let PlayerTalentTree::Warrior(ref mut w) = player.talents {
        w.taunt_unlocked = true;
        w.war_fury_lvl = 5;
        w.executioner_lvl = 5;
    }
    
    if let PlayerTalentTree::Rogue(ref mut r) = player.talents {
        r.shadow_step_unlocked = true;
        r.flurry_blades_lvl = 5;
    }

    
    if let PlayerTalentTree::Mage(ref mut mage) = player.talents {
        if let Some(crate::talents::ElementalDpsTree::Fire(ref mut fire)) = mage.dps_tree {
            fire.fireball = 5;
            fire.ignite = 5;
            fire.firestorm = 5;
        }
    }
    let mut turn_number = 1;

    loop {
        println!("--- 🔄 TURN {} ---", turn_number);
        println!("🧑 {} | HP: {:.0}/{:.0} | Mana: {:.0}/{:.0}", 
            player.name, player.stats.current_hp, player.stats.max_hp, player.stats.current_mana, player.stats.max_mana);
        println!("👾 {} | HP: {:.0}/{:.0}", 
            enemy.name, enemy.current_hp, enemy.max_hp);

        let abilities = player.get_available_abilities();
        println!("\nChoose your action:");
        for (i, ability) in abilities.iter().enumerate() {
            println!("  [{}] {}", i + 1, ability);
        }

        print!("\n> ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input! You fumbled and lost your turn.");
                0
            }
        };

        if choice > 0 && choice <= abilities.len() {
            let selected_ability = &abilities[choice - 1];
            println!("\n▶️ You used: {}", selected_ability);
            player.use_ability(selected_ability, enemy);
        }
if enemy.ignite_turns > 0 {
            let burn_dmg = 35.0; 
            enemy.current_hp -= burn_dmg;
            enemy.ignite_turns -= 1;
            println!("🔥 {} takes {:.0} Burn damage! ({} turns left)", enemy.name, burn_dmg, enemy.ignite_turns);
        }

        if enemy.poison_stacks > 0 {
            let poison_dmg = (enemy.poison_stacks as f64) * 15.0; 
            enemy.current_hp -= poison_dmg;
            println!("🤢 {} takes {:.0} Poison damage from {} stacks!", enemy.name, poison_dmg, enemy.poison_stacks);
        }
        
        if enemy.is_dead() {
            println!("\n🎉 VICTORY! {} succumbed to their wounds!", enemy.name);
            player.defeat_enemy(enemy);
            break;
        }


println!("\n▶️ {}'s turn!", enemy.name);
        
        
        if enemy.is_frozen {
            println!("🥶 {} is frozen solid and skips their turn!", enemy.name);
            enemy.is_frozen = false; 
        } else {
            
            let enemy_base_dmg = 15.0; 
            let damage_taken = (enemy_base_dmg - player.stats.armor).max(1.0);
            
            player.stats.current_hp -= damage_taken;
            println!("💥 {} attacks {} for {:.0} damage!", enemy.name, player.name, damage_taken);

            
            if player.stats.current_hp <= 0.0 {
                println!("\n💀 GAME OVER! {} was defeated by {}...", player.name, enemy.name);
                break;
            }
        }

        player.tick_cooldowns();
        player.regenerate();
        enemy.regenerate();

        println!("✨ Turn ends: HP/Mana regenerated.");
        //println!("\n--------------------------------------------\n");
        println!("\n--------------------------------------------\n");
        turn_number += 1;
    }
}

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║    ⚔️ KOKTEL_TALES MAX-LEVEL ARENA ⚔️     ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nSelect your specialization (All characters are Level 60):");
    
    println!("\n🛡️ WARRIOR:");
    println!("  [1] Arms Warrior (Melee DPS)");
    println!("  [2] Protection Warrior (Tank - High Armor/HP)");
    
    println!("\n🪄 MAGE:");
    println!("  [3] Ice Mage (Elemental CC)");
    println!("  [4] Void Mage (DoT Damage)");
    println!("  [5] Fire Mage (Shadow/Mana Burn)");
    println!("  [6] Poison Mage (Damage over Time)"); 

    println!("\n🗡️ ROGUE:");
    println!("  [7] Assassination Rogue (Burst/Poison)");
    println!("  [8] Duelist Rogue (Evasion/Fast Strikes)");

    print!("\n> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut player = match input.trim() {
        "1" => Player::new("Garrosh", PlayerClass::Warrior, PlayerSpec::WarriorDPS),
        "2" => Player::new("Muradin", PlayerClass::Warrior, PlayerSpec::WarriorTank),
        "3" => {
            let mut mage = Player::new("Jaina", PlayerClass::Mage, PlayerSpec::MageElemental);
            mage.choose_elemental_spec("ice");
            mage
        },
        "4" => {
            let mut mage = Player::new("Guldan", PlayerClass::Mage, PlayerSpec::MageElemental);
            mage.choose_elemental_spec("void");
            mage
        },
        "5" => {
            let mut mage = Player::new("Kaelthas", PlayerClass::Mage, PlayerSpec::MageElemental); 
            mage.choose_elemental_spec("fire");
            mage
        }, 
        "6" => {
            let mut mage = Player::new("Kaelthas", PlayerClass::Mage, PlayerSpec::MageElemental); 
            mage.choose_elemental_spec("poison"); 
            mage
        },
        "7" => Player::new("Valeera", PlayerClass::Rogue, PlayerSpec::RogueAssassin), 
        "8" => Player::new("Garona", PlayerClass::Rogue, PlayerSpec::RogueDuelist), 
        _ => {
            println!("\n❌ Invalid choice! Defaulting to Ice Mage.");
            let mut default_mage = Player::new("Jaina", PlayerClass::Mage, PlayerSpec::MageElemental);
            default_mage.choose_elemental_spec("ice");
            default_mage
        }
    };


    for _ in 0..59 {
        player.level_up();
    }


    player.talent_points = 20; 
    

    if let PlayerTalentTree::Mage(ref mage) = player.talents {
        if let Some(ref dps_tree) = mage.dps_tree {
            match dps_tree {
                talents::ElementalDpsTree::Ice(_) => {
                    player.upgrade_mage_dps_talent("frostbolt");
                    player.upgrade_mage_dps_talent("ice_lance");
                }
                talents::ElementalDpsTree::Void(_) => {
                    player.upgrade_mage_dps_talent("void_bolt"); 
                    player.upgrade_mage_dps_talent("void_drain"); 
                }
                talents::ElementalDpsTree::Fire(_) => {
                    player.upgrade_mage_dps_talent("fireball");
                }
                talents::ElementalDpsTree::Poison(_) => {
                    player.upgrade_mage_dps_talent("venom_strike");
                }
            }
        }
    }
    player.stats.current_hp = player.stats.max_hp;
    player.stats.current_mana = player.stats.max_mana;
    if let PlayerTalentTree::Warrior(ref mut w) = player.talents {
        if player.spec == PlayerSpec::WarriorTank {
            w.taunt_unlocked = true; 
            w.iron_fortress_lvl = 5; 
        } else {
            w.war_fury_lvl = 5; 
            w.executioner_lvl = 5;
        }
    }

    
    if let PlayerTalentTree::Rogue(ref mut r) = player.talents {
        if player.spec == PlayerSpec::RogueDuelist {
            r.parry_lvl = 5; 
        }
    }
    
if player.spec == PlayerSpec::RogueAssassin {
    if let PlayerTalentTree::Rogue(ref mut r) = player.talents {
        r.shadow_step_unlocked = true; 
        r.flurry_blades_lvl = 5;
    }
}

if player.spec == PlayerSpec::RogueDuelist {
    if let PlayerTalentTree::Rogue(ref mut r) = player.talents {
        r.parry_lvl = 5; 
    }
}
if player.class == PlayerClass::Warrior {
    if let PlayerTalentTree::Warrior(ref mut w) = player.talents {
        if player.spec == PlayerSpec::WarriorTank {
            w.iron_fortress_lvl = 5;
            w.ironwill_lvl = 3;
            w.taunt_unlocked = true;
        } else {
            w.berserker_crit_lvl = 5;
            w.true_strike_unlocked = true;
            w.war_fury_lvl = 5;
            w.executioner_lvl = 5;
        }
    }
    
    player.apply_warrior_passives();
}
    println!("\n✅ Character initialized at Level 60!");
    println!("📊 HP: {:.0} | Mana: {:.0} | Armor: {:.2}", player.stats.max_hp, player.stats.max_mana, player.stats.armor);
    println!("📈 HP Regen: {:.1}/turn | Mana Regen: {:.1}/turn", player.stats.hp_regen, player.stats.mana_regen);


    let mut orc = Enemy::new("Elite Orc Overlord", 8000.0, 10.0, 150.0, 80);
    

    interactive_combat(&mut player, &mut orc);
}

#[cfg(test)]
mod tests {
    use crate::mechanics::{Player, PlayerClass, PlayerSpec, mana_cost};
    use crate::combat::Enemy;
    use crate::talents::PlayerTalentTree;

    fn make_enemy(hp: f64) -> Enemy {
        Enemy::new("Dummy", hp, 0.0, 0.0, 0)
    }

    
    #[test]
    fn test_warrior_dps_base_stats() {
        let p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        assert_eq!(p.stats.strength,     30.0);
        assert_eq!(p.stats.agility,      18.0);
        assert_eq!(p.stats.intelligence, 15.0);
        assert_eq!(p.level, 1);
        assert_eq!(p.talent_points, 0);
        assert_eq!(p.gold, 0);
    }

    #[test]
    fn test_warrior_tank_base_stats() {
        let p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorTank);
        assert_eq!(p.stats.strength, 32.0);
        assert_eq!(p.stats.agility,  15.0);
        let expected_armor = 15.0 * 0.65;
        assert!((p.stats.armor - expected_armor).abs() < 0.01);
    }

    #[test]
    fn test_tank_has_more_armor_than_dps() {
        let tank = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorTank);
        let dps  = Player::new("D", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        assert!(tank.stats.armor > dps.stats.armor,
            "Tank {:.2} should > DPS {:.2}", tank.stats.armor, dps.stats.armor);
    }

    // ── Mana ─────────────────────────────────────────────────
    #[test]
    fn test_mana_costs_defined() {
        assert_eq!(mana_cost("basic_attack"), 0.0);
        assert_eq!(mana_cost("frostbolt"),    20.0);
        assert_eq!(mana_cost("fireball"),     25.0);
        assert_eq!(mana_cost("void_bolt"),    18.0);
        assert_eq!(mana_cost("holy_heal"),    30.0);
        assert_eq!(mana_cost("shadow_step"),  15.0);
    }

    #[test]
    fn test_frostbolt_deducts_mana() {
        let mut mage = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        mage.choose_elemental_spec("ice");
        mage.talent_points = 1;
        mage.upgrade_mage_dps_talent("frostbolt");
        let mana_before = mage.stats.current_mana;
        let mut enemy = make_enemy(1000.0);
        mage.use_ability("frostbolt", &mut enemy);
        assert_eq!(mage.stats.current_mana, mana_before - mana_cost("frostbolt"));
    }

    #[test]
    fn test_not_enough_mana_blocks_cast() {
        let mut mage = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        mage.choose_elemental_spec("ice");
        mage.talent_points = 1;
        mage.upgrade_mage_dps_talent("frostbolt");
        mage.stats.current_mana = 5.0;
        let mut enemy = make_enemy(1000.0);
        mage.use_ability("frostbolt", &mut enemy);
        assert_eq!(mage.stats.current_mana, 5.0);
        assert_eq!(enemy.current_hp, 1000.0);
    }

    #[test]
    fn test_basic_attack_costs_no_mana() {
        let mut warrior = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let mana_before = warrior.stats.current_mana;
        let mut enemy = make_enemy(500.0);
        warrior.use_ability("basic_attack", &mut enemy);
        assert_eq!(warrior.stats.current_mana, mana_before);
    }

    // ── Combat ───────────────────────────────────────────────
    #[test]
    fn test_basic_attack_deals_damage() {
        let mut warrior = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let mut enemy = make_enemy(500.0);
        warrior.use_ability("basic_attack", &mut enemy);
        assert!(enemy.current_hp < 500.0);
    }

    #[test]
    fn test_ice_lance_bonus_on_frozen() {
        let mut mage = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        mage.choose_elemental_spec("ice");
        mage.talent_points = 2;
        mage.upgrade_mage_dps_talent("frostbolt");
        mage.upgrade_mage_dps_talent("ice_lance");
        mage.stats.current_mana = 999.0;

        let mut enemy_normal = make_enemy(1000.0);
        mage.use_ability("ice_lance", &mut enemy_normal);
        let dmg_normal = 1000.0 - enemy_normal.current_hp;

        mage.tick_cooldowns(); 

        let mut enemy_frozen = make_enemy(1000.0);
        enemy_frozen.is_frozen = true;
        mage.stats.current_mana = 999.0;
        mage.use_ability("ice_lance", &mut enemy_frozen);
        let dmg_frozen = 1000.0 - enemy_frozen.current_hp;

        assert!(dmg_frozen > dmg_normal,
            "Frozen ({:.1}) should > Normal ({:.1})", dmg_frozen, dmg_normal);
    }

    #[test]
    fn test_enemy_gold_and_exp_reward() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let mut enemy = Enemy::new("Goblin", 1.0, 0.0, 75.0, 50);
        enemy.current_hp = 0.0;
        p.defeat_enemy(&enemy);
        assert_eq!(p.gold, 50);
    }

    // ── Level Up ─────────────────────────────────────────────
    #[test]
    fn test_level_up_grants_talent_point() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        p.gain_exp(9999.0);
        assert!(p.talent_points >= 1);
    }

    #[test]
    fn test_level_up_increases_stats() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let hp_before  = p.stats.max_hp;
        let str_before = p.stats.strength;
        p.level_up();
        assert!(p.stats.max_hp   > hp_before);
        assert!(p.stats.strength > str_before);
    }

    #[test]
    fn test_multi_level_up_loop() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        p.gain_exp(9999.0);
        assert!(p.level > 3);
        assert_eq!(p.talent_points, p.level - 1);
    }

    // ── Cooldown ─────────────────────────────────────────────
    #[test]
    fn test_frostbolt_cooldown() {
        let mut mage = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        mage.choose_elemental_spec("ice");
        mage.talent_points = 1;
        mage.upgrade_mage_dps_talent("frostbolt");
        mage.stats.current_mana = 999.0;

        let mut enemy = make_enemy(1000.0);
        mage.use_ability("frostbolt", &mut enemy);       // cast 1 — OK

        let mana_after_first = mage.stats.current_mana;
        mage.use_ability("frostbolt", &mut enemy);       // cast 2 — blocked
        assert_eq!(mage.stats.current_mana, mana_after_first, "Should be blocked by cooldown");

        mage.tick_cooldowns();
        mage.use_ability("frostbolt", &mut enemy);       // cast 3 — OK again
        assert!(mage.stats.current_mana < mana_after_first, "Should fire after cooldown");
    }

    // ── Rogue Flurry ─────────────────────────────────────────
    #[test]
    fn test_rogue_flurry_deals_more_than_single_hit() {
        let mut rogue_flurry = Player::new("S", PlayerClass::Rogue, PlayerSpec::RogueAssassin);
        let mut rogue_normal = Player::new("S", PlayerClass::Rogue, PlayerSpec::RogueAssassin);
        rogue_normal.stats.agility = 0.0; // extra attack chance = 0

        if let PlayerTalentTree::Rogue(ref mut r) = rogue_flurry.talents {
            r.flurry_blades_lvl = 1;
        }

        let mut enemy_a = make_enemy(5000.0);
        let mut enemy_b = make_enemy(5000.0);
        for _ in 0..10 {
            rogue_flurry.use_ability("basic_attack", &mut enemy_a);
            rogue_normal.use_ability("basic_attack", &mut enemy_b);
        }

        let dmg_flurry = 5000.0 - enemy_a.current_hp;
        let dmg_normal = 5000.0 - enemy_b.current_hp;
        assert!(dmg_flurry > dmg_normal,
            "Flurry ({:.1}) should > Normal ({:.1})", dmg_flurry, dmg_normal);
    }
}