mod mechanics;
mod talents;
mod combat;

use mechanics::{Player, PlayerClass};
use combat::Enemy;
use crate::talents::PlayerTalentTree;

fn main() {
    let mut mage = Player::new("Jaina", PlayerClass::Mage);
    let mut weak_goblin = Enemy::new("Scout Goblin", 20.0, 0.0, 120.0, 50);

    println!("Initial State -> Level: {}, Gold: {}, Talent Points: {}", mage.level, mage.gold, mage.talent_points);

    println!("\n--- ⚔️ COMBAT WITH GOBLIN BEGINS ---");
    
    
    while !weak_goblin.is_dead() {
        mage.use_ability("basic_attack", &mut weak_goblin);
        
        
        if weak_goblin.is_dead() {
            mage.defeat_enemy(&weak_goblin);
            break;
        }
    }

    println!("\nPost-Combat State -> Level: {}, Gold: {}, Talent Points: {}", mage.level, mage.gold, mage.talent_points);
    println!("--- COMBAT ENDED ---\n");
    let mut rogue = Player::new("Shadow", PlayerClass::Rogue);
    
    if let talents::PlayerTalentTree::Rogue(ref mut r) = rogue.talents {
        r.flurry_blades_lvl = 1; 
    }

    let mut orc = Enemy::new("Elite Grunt", 200.0, 5.0, 100.0, 50);

    println!("--- ⚔️ ROGUE COMBAT TEST ---");
    println!("{} is attacking with Flurry Blades enabled!", rogue.name);

    
    rogue.use_ability("basic_attack", &mut orc);

    println!("\nFinal Orc HP: {:.1}", orc.current_hp);
    println!("\n--- COMBAT ENDED ---\n");
    let mut rogue = Player::new("Shadow", PlayerClass::Rogue);
    rogue.stats.crit_chance = 0.5; 
    
    if let talents::PlayerTalentTree::Rogue(ref mut r) = rogue.talents {
        r.flurry_blades_lvl = 1; 
    }

    let mut target = Enemy::new("Training Dummy", 500.0, 0.0, 0.0, 0);

    println!("--- ⚔️ COMBAT TEST: ROGUE FLURRY + CRIT ---");
    println!("Stats: Agility: {:.0}, Crit Chance: {:.0}%", rogue.stats.agility, rogue.stats.crit_chance * 100.0);

    rogue.use_ability("basic_attack", &mut target);

    println!("\nTarget Dummy final HP: {:.1}", target.current_hp);
    println!("\n--- COMBAT ENDED ---\n");

    let mut warrior = Player::new("Thor", PlayerClass::Warrior);
    warrior.stats.agility = 50.0; 
    warrior.stats.crit_chance = 0.8; 
    
    let mut dummy = Enemy::new("Training Dummy", 500.0, 10.0, 0.0, 0);
    println!("--- ⚔️ WARRIOR COMBAT TEST (Extra Attack & Crit) ---");
    println!("Stats: Agility: {:.0}, Crit Chance: {:.0}%", warrior.stats.agility, warrior.stats.crit_chance * 100.0);
    warrior.use_ability("basic_attack", &mut dummy);
    println!("\nTarget Dummy final HP: {:.1}", dummy.current_hp);
    println!("\n--- COMBAT ENDED ---\n");
    // Add more tests and scenarios as needed

    let mut rogue = Player::new("Shadow", PlayerClass::Rogue);
    let mut warrior = Player::new("Thor", PlayerClass::Warrior);


    rogue.stats.agility = 10.0;
    warrior.stats.strength = 10.0;
    

    if let PlayerTalentTree::Rogue(ref mut r) = rogue.talents {
        r.flurry_blades_lvl = 1;
    }

    
    rogue.update_derived_stats();
    warrior.update_derived_stats();

    println!("--- 🛡️ PRE-LEVEL UP STATS ---");
    println!("Rogue HP: {:.1}, Armor: {:.2}, Dmg: {:.1}", rogue.stats.max_hp, rogue.stats.armor, rogue.base_damage());
    println!("Warrior HP: {:.1}, Armor: {:.2}, Dmg: {:.1}", warrior.stats.max_hp, warrior.stats.armor, warrior.base_damage());

    println!("\n--- 🆙 LEVELING UP ---");
    rogue.level_up(); 
    warrior.level_up();

    println!("\n--- ⚔️ POST-LEVEL UP & COMBAT TEST ---");
    println!("Rogue (New Stats) -> HP: {:.1}, Armor: {:.2}, Dmg: {:.1}", rogue.stats.max_hp, rogue.stats.armor, rogue.base_damage());
    println!("Warrior (New Stats) -> HP: {:.1}, Armor: {:.2}, Dmg: {:.1}", warrior.stats.max_hp, warrior.stats.armor, warrior.base_damage());

    
    let mut enemy = Enemy::new("Target Dummy", 1000.0, 5.0, 0.0, 0);
    
    println!("\n--- 🥊 COMBAT EXECUTION ---");
    rogue.use_ability("basic_attack", &mut enemy);
    warrior.use_ability("basic_attack", &mut enemy);
}