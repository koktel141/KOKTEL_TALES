use crate::mechanics::PlayerClass;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub class_restriction: PlayerClass,
    pub strength_bonus: f64,
    pub agility_bonus: f64,
    pub intelligence_bonus: f64,
    pub armor_bonus: f64,
    pub price: u32,
}

impl Item {
    pub fn generate_loot(player_class: PlayerClass, wave: u32) -> Self {
        //let mut rng = rand::thread_rng();
        let tier = (wave / 3) + 1;
        
        match player_class {
            PlayerClass::Warrior => Item {
                name: format!("Vanguard Plate Armor T{}", tier),
                class_restriction: PlayerClass::Warrior,
                strength_bonus: (tier as f64) * 5.0,
                agility_bonus: 0.0,
                intelligence_bonus: 0.0,
                armor_bonus: (tier as f64) * 12.0,
                price: tier * 40,
            },
            PlayerClass::Rogue => Item {
                name: format!("Shadow-Stalker Vest T{}", tier),
                class_restriction: PlayerClass::Rogue,
                strength_bonus: 0.0,
                agility_bonus: (tier as f64) * 6.0,
                intelligence_bonus: 0.0,
                armor_bonus: (tier as f64) * 6.0,
                price: tier * 45,
            },
            PlayerClass::Mage => Item {
                name: format!("Archmage Robe T{}", tier),
                class_restriction: PlayerClass::Mage,
                strength_bonus: 0.0,
                agility_bonus: 0.0,
                intelligence_bonus: (tier as f64) * 8.0,
                armor_bonus: (tier as f64) * 3.0,
                price: tier * 50,
            },
        }
    }
}