use crate::combat::Enemy;

pub struct WaveManager {
    pub current_wave: u32,
    pub total_boss_killed: u32,
}

impl WaveManager {
    pub fn new() -> Self {
        Self {
            current_wave: 1,
            total_boss_killed: 0,
        }
    }

    pub fn waves_until_next_boss(&self) -> u32 {
        if self.current_wave % 5 == 0 {
            0
        } else {
            5 - (self.current_wave % 5)
        }
    }

    pub fn spawn_wave_enemy(&self) -> Enemy {
        let wave = self.current_wave;
        
        if wave % 5 == 0 {
            
            Enemy::new(
                &format!("Archdemon Overlord (Boss Tier {})", wave / 5),
                wave as f64 * 350.0,
                wave as f64 * 12.0,
                wave as f64 * 150.0,
                wave * 100,
            )
        } else if wave >= 5 && wave <= 10 {
            
            if wave % 2 == 0 {
                Enemy::new("Elite Shadow Assassin", wave as f64 * 140.0, wave as f64 * 6.0, wave as f64 * 45.0, wave * 25)
            } else {
                Enemy::new("Corrupted Golem", wave as f64 * 190.0, wave as f64 * 10.0, wave as f64 * 50.0, wave * 30)
            }
        } else {
            
            Enemy::new("Cursed Skeleton", wave as f64 * 80.0, wave as f64 * 3.0, wave as f64 * 20.0, wave * 15)
        }
    }
}