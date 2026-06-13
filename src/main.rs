mod mechanics;
mod talents;
mod combat;

use mechanics::{Player, PlayerClass};
use combat::Enemy;

fn main() {
    
    let mut wizard = Player::new("Khadgar", PlayerClass::Mage);
    
    
    let mut orc = Enemy::new("Iron Orc", 250.0, 30.0);
    
    
    wizard.talent_points = 5;
    wizard.choose_elemental_spec("ice");
    
    
    wizard.upgrade_mage_dps_talent("frostbolt");
    wizard.upgrade_mage_dps_talent("ice_lance");
    
    println!("\n--- ⚔️ COMBAT BEGINS ⚔️ ---");
    
    
    wizard.cast_mage_spell("frostbolt", &mut orc);
    
    
    wizard.cast_mage_spell("ice_lance", &mut orc);
    
    
    let mut warrior = Player::new("Grommash", PlayerClass::Warrior);
    let mut orc_enemy = Enemy::new("Target Dummy", 500.0, 10.0);

    println!("--- 🎛️ UI Rendering for Warrior ---");
    
    let warrior_ui = warrior.get_available_abilities();
    println!("Buttons to render on screen for Warrior: {:?}", warrior_ui); 
    
    warrior.use_ability("fireball", &mut orc_enemy); 


    println!("\n--- 🎛️ UI Rendering for Level up Mage ---");
    
    let mut mage = Player::new("Jaina", PlayerClass::Mage);
    mage.talent_points = 2;
    mage.choose_elemental_spec("ice");
    mage.upgrade_mage_dps_talent("frostbolt"); 

    let mage_ui = mage.get_available_abilities();
    println!("Buttons to render on screen for Mage: {:?}", mage_ui);



    mage.use_ability("frostbolt", &mut orc_enemy);

    let mut mage = Player::new("Jaina", PlayerClass::Mage);
    let mut orc = Enemy::new("Elite Grunt", 300.0, 20.0);

    mage.talent_points = 5;
    mage.choose_elemental_spec("ice");
    mage.upgrade_mage_dps_talent("frostbolt");
    mage.upgrade_mage_dps_talent("ice_lance");

    println!("\n--- 🟢 ROUND 1 ---");
    mage.use_ability("ice_lance", &mut orc); 
    
    println!("\n--- 🟢 ROUND 2 (Spamming Ice Lance) ---");
    mage.use_ability("ice_lance", &mut orc); 
    
    println!("\n--- ⏳ TURNS PASSING ---");
    mage.tick_cooldowns(); 
    mage.tick_cooldowns(); 
    mage.tick_cooldowns(); 

    println!("\n--- 🟢 ROUND 5 ---");
    mage.use_ability("ice_lance", &mut orc); 
}