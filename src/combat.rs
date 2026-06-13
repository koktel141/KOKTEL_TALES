#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub max_hp: f64,
    pub current_hp: f64,
    pub armor: f64,
    
    pub is_frozen: bool,
    pub poison_stacks: u32,
    pub ignite_turns: u32, 
}

impl Enemy {
    pub fn new(name: &str, hp: f64, armor: f64) -> Self {
        Self {
            name: name.to_string(),
            max_hp: hp,
            current_hp: hp,
            armor,
            is_frozen: false,
            poison_stacks: 0,
            ignite_turns: 0,
        }
    }

    pub fn take_damage(&mut self, amount: f64) {
        let reduction = (self.armor / (self.armor + 100.0)) * amount;
        let final_damage = (amount - reduction).max(1.0);
        
        self.current_hp = (self.current_hp - final_damage).max(0.0);
        println!("💥 {} took {:.1} damage (Blocked {:.1} via Armor). HP: {:.0}/{:.0}", 
            self.name, final_damage, reduction, self.current_hp, self.max_hp);
    }
}