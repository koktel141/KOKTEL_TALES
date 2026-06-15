#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub max_hp: f64,
    pub current_hp: f64,
    pub armor: f64,
    pub exp_reward: f64,
    pub gold_reward: u32,
    pub is_frozen: bool,
    pub poison_stacks: u32,
    pub ignite_turns: u32,
    pub hp_regen: f64,
}

impl Enemy {
    pub fn new(name: &str, hp: f64, armor: f64, exp_reward: f64, gold_reward: u32) -> Self {
        Self {
            name: name.to_string(),
            max_hp: hp,
            current_hp: hp,
            armor,
            exp_reward,
            gold_reward,
            is_frozen: false,
            poison_stacks: 0,
            ignite_turns: 0,
            hp_regen: hp * 0.01, // 1% of max HP regen per turn
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current_hp <= 0.0
    }

    /// Base attack damage — scales with enemy HP (tougher enemies hit harder)
    pub fn base_attack(&self) -> f64 {
        self.max_hp * 0.06
    }

    pub fn take_damage(&mut self, amount: f64) {
        let reduction = (self.armor / (self.armor + 100.0)) * amount;
        let final_damage = (amount - reduction).max(1.0);
        self.current_hp = (self.current_hp - final_damage).max(0.0);
        println!(
            "💥 {} took {:.1} damage (Blocked {:.1} via Armor). HP: {:.0}/{:.0}",
            self.name, final_damage, reduction, self.current_hp, self.max_hp
        );
    }

    pub fn take_true_damage(&mut self, amount: f64) {
        self.current_hp = (self.current_hp - amount).max(0.0);
        println!(
            "🌀 {} took {:.1} TRUE damage. HP: {:.0}/{:.0}",
            self.name, amount, self.current_hp, self.max_hp
        );
    }

    pub fn regenerate(&mut self) {
        if self.current_hp > 0.0 {
            self.current_hp = (self.current_hp + self.hp_regen).min(self.max_hp);
        }
    }
}