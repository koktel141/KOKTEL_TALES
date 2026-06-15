use crate::mechanics::{Player, PlayerClass, StatModifier};
use rand::Rng;

pub struct ShopItem {
    pub name: String,
    pub price: u32,
    pub modifier: StatModifier,
}

pub struct Shop {
    pub items_for_sale: Vec<ShopItem>,
}

impl Shop {
    pub fn generate_shop_stock(player: &Player, wave: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut items = vec![];

        // Slot 1: Class-specific offensive scaling weapon
        let (w_name, w_mod, w_price) = match player.class {
            PlayerClass::Warrior => (format!("Vanguard Greatsword (+{} Str)", wave * 4), StatModifier::AddStrength((wave * 4) as f64), 15 + wave * 10),
            PlayerClass::Rogue => (format!("Deadly Stiletto (+{} Agi)", wave * 4), StatModifier::AddAgility((wave * 4) as f64), 15 + wave * 10),
            PlayerClass::Mage => (format!("Archmage Staff (+{} Int)", wave * 4), StatModifier::AddIntelligence((wave * 4) as f64), 15 + wave * 10),
        };
        items.push(ShopItem { name: w_name, price: w_price, modifier: w_mod });

        // Slot 2: Defensive armor or maximum health expansion
        let armor_val = 5 + wave * 3;
        let hp_val = 20 + wave * 15;
        if rng.gen_bool(0.5) {
            items.push(ShopItem {
                name: format!("Reinforced Platemail (+{} Armor)", armor_val),
                price: 10 + wave * 8,
                modifier: StatModifier::AddArmor(armor_val as f64),
            });
        } else {
            items.push(ShopItem {
                name: format!("Vitality Amulet (+{} Max HP)", hp_val),
                price: 12 + wave * 8,
                modifier: StatModifier::AddMaxHp(hp_val as f64),
            });
        }

        // Slot 3: Utility attributes (Critical rating or Mana reservoirs)
        let crit_val = 0.02 + (wave as f64 * 0.01);
        let mana_val = 15.0 + (wave as f64 * 10.0);
        if rng.gen_bool(0.5) {
            items.push(ShopItem {
                name: format!("Sharp Eye Ring (+{:.0}% Crit)", crit_val * 100.0),
                price: 20 + wave * 12,
                modifier: StatModifier::AddCritChance(crit_val),
            });
        } else {
            items.push(ShopItem {
                name: format!("Ancient Mana Crystal (+{} Max Mana)", mana_val),
                price: 10 + wave * 6,
                modifier: StatModifier::AddMaxMana(mana_val),
            });
        }

        Shop { items_for_sale: items }
    }

pub fn buy_item(&mut self, player: &mut Player, index: usize) {
    if index < self.items_for_sale.len() {
        let item = &self.items_for_sale[index];
        
        // Check if the player has enough gold
        if player.gold >= item.price {
            player.gold -= item.price;

            // Apply the stat modifier to the player's stats
            match item.modifier {
                StatModifier::AddStrength(val) => {
                    player.stats.strength += val;
                }
                StatModifier::AddAgility(val) => {
                    player.stats.agility += val;
                }
                StatModifier::AddIntelligence(val) => {
                    player.stats.intelligence += val;
                }
                StatModifier::AddMaxHp(val) => {
                    player.stats.max_hp += val;
                    player.stats.current_hp += val; // Also heal for the added amount
                }
                StatModifier::AddMaxMana(val) => {
                    player.stats.max_mana += val;
                    player.stats.current_mana += val;
                }
                StatModifier::AddArmor(val) => {
                    player.stats.armor += val;
                }
                StatModifier::AddBlockChance(val) => {
                    player.stats.block_chance += val;
                }
                StatModifier::AddBlockValue(val) => {
                    player.stats.block_value += val;
                }
                StatModifier::AddCritChance(val) => {
                    player.stats.crit_chance += val;
                }
                StatModifier::AddHpRegen(val) => {
                    player.stats.hp_regen += val;
                }
                StatModifier::AddManaRegen(val) => {
                    player.stats.mana_regen += val;
                }
                StatModifier::NoEffect => {}
            }
            self.items_for_sale.remove(index);
        }
    }
}
}