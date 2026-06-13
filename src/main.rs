mod mechanics;
mod talents;
mod combat;

use mechanics::{Player, PlayerClass};
use combat::Enemy;

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
}