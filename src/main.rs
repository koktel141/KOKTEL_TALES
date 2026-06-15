mod mechanics;
mod talents;
mod combat;
mod shop;

use macroquad::prelude::*;
use mechanics::{Player, PlayerClass, PlayerSpec};
use combat::Enemy;
use talents::PlayerTalentTree;

// ── Window config ────────────────────────────────────────────
fn window_conf() -> Conf {
    Conf {
        window_title: "Koktel Tales".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

const W: f32 = 800.0;
const H: f32 = 600.0;
const GROUND_Y: f32 = 420.0;
const S: f32 = 1.8; // sprite scale

// ── Screens ──────────────────────────────────────────────────
#[derive(PartialEq, Clone)]
enum Screen {
    MainMenu,
    ClassSelect,
    Combat,
    Victory,
    GameOver,
    LevelUp,
    Shop,
}

// ── Class options ─────────────────────────────────────────────
struct ClassOption {
    label: &'static str,
    class: PlayerClass,
    spec: PlayerSpec,
    element: Option<&'static str>,
    color: Color,
    first_talent: Option<&'static str>,
}

fn class_options() -> Vec<ClassOption> {
    vec![
        ClassOption { label: "Warrior DPS  — STR scaling, high crit",      class: PlayerClass::Warrior, spec: PlayerSpec::WarriorDPS,    element: None,          first_talent: None,             color: Color::from_rgba(210,  80, 50, 255) },
        ClassOption { label: "Warrior Tank — DEF scaling, high armor/HP",   class: PlayerClass::Warrior, spec: PlayerSpec::WarriorTank,   element: None,          first_talent: None,             color: Color::from_rgba(100, 150, 200, 255) },
        ClassOption { label: "Rogue Assassin — AGI scaling, flurry hits",   class: PlayerClass::Rogue,   spec: PlayerSpec::RogueAssassin, element: None,          first_talent: None,             color: Color::from_rgba(100,  80, 160, 255) },
        ClassOption { label: "Rogue Duelist  — Parry & counter strikes",    class: PlayerClass::Rogue,   spec: PlayerSpec::RogueDuelist,  element: None,          first_talent: None,             color: Color::from_rgba( 80, 140, 120, 255) },
        ClassOption { label: "Mage Ice    — Freeze, Shatter combo",         class: PlayerClass::Mage,    spec: PlayerSpec::MageElemental, element: Some("ice"),   first_talent: Some("frostbolt"), color: Color::from_rgba(100, 180, 230, 255) },
        ClassOption { label: "Mage Fire   — Burn DoT, AoE burst",           class: PlayerClass::Mage,    spec: PlayerSpec::MageElemental, element: Some("fire"),  first_talent: Some("fireball"),  color: Color::from_rgba(230, 100,  30, 255) },
        ClassOption { label: "Mage Void   — True Damage, Silence",          class: PlayerClass::Mage,    spec: PlayerSpec::MageElemental, element: Some("void"),  first_talent: Some("void_bolt"), color: Color::from_rgba(140,  60, 200, 255) },
        ClassOption { label: "Mage Poison — Stacking DoT, Execute",         class: PlayerClass::Mage,    spec: PlayerSpec::MageElemental, element: Some("poison"),first_talent: Some("venom_strike"),color: Color::from_rgba( 80, 180,  60, 255) },
    ]
}

// ── GameState ─────────────────────────────────────────────────
struct GameState {
    screen:       Screen,
    player:       Option<Player>,
    enemy:        Option<Enemy>,
    turn:         u32,
    wave:         u32,
    log:          Vec<String>,
    abilities:    Vec<String>,
    cursor:       usize, 
    shop:         Option<crate::shop::Shop>,
}

impl GameState {
    fn new() -> Self {
        Self {
            screen:    Screen::MainMenu,
            player:    None,
            enemy:     None,
            turn:      1,
            wave:      1,
            log:       Vec::new(),
            abilities: Vec::new(),
            cursor:    0,
            shop:      None,
        }
    }

    fn add_log(&mut self, msg: impl Into<String>) {
        self.log.push(msg.into());
        while self.log.len() > 7 { self.log.remove(0); }
    }

    // ── Wave spawner ─────────────────────────────────────────
fn spawn_enemy(&self) -> Enemy {
        let is_boss = self.wave % 5 == 0;
        // Dynamic scaling: enemies get stronger as waves increase
        let scale = 1.0 + ((self.wave as f64 - 1.0) * 0.25);

        if is_boss {
            Enemy::new(
                &format!("Ancient Golem Boss (Wave {})", self.wave),
                2000.0 * scale,
                60.0 * scale,
                800.0 * scale,
                500 + (self.wave * 50)
            )
        } else {
            let mod_wave = self.wave % 5;
            let (name, hp, atk, exp, gold) = match mod_wave {
                1 => ("Goblin Scout", 80.0, 5.0, 75.0, 30),
                2 => ("Orc Warrior", 200.0, 12.0, 150.0, 60),
                3 => ("Dark Knight", 450.0, 22.0, 280.0, 110),
                4 => ("Fire Dragon", 1000.0, 38.0, 500.0, 250),
                _ => ("Unknown", 100.0, 10.0, 100.0, 50),
            };

            Enemy::new(
                &format!("{} (Wave {})", name, self.wave),
                hp * scale,
                atk * scale,
                exp * scale,
                gold + (self.wave * 10)
            )
        }
    }

    // ── Build player from class option ────────────────────────
    fn build_player(opt: &ClassOption) -> Player {
        let mut p = Player::new("Hero", opt.class, opt.spec);
        if let Some(elem) = opt.element {
            p.choose_elemental_spec(elem);
        }
        if let Some(talent) = opt.first_talent {
            p.talent_points = 1;
            p.upgrade_mage_dps_talent(talent);
        }
        // Give Rogue assassin branch its first talent
        if opt.spec == PlayerSpec::RogueAssassin {
            if let PlayerTalentTree::Rogue(ref mut r) = p.talents {
                r.flurry_blades_lvl = 1;
            }
        }
        // Give Warrior DPS its first talent
        if opt.spec == PlayerSpec::WarriorDPS {
            if let PlayerTalentTree::Warrior(ref mut w) = p.talents {
                w.war_fury_lvl = 1;
            }
        }
        // Give Warrior Tank its first talent
        if opt.spec == PlayerSpec::WarriorTank {
            if let PlayerTalentTree::Warrior(ref mut w) = p.talents {
                w.taunt_unlocked = true;
            }
        }
        p
    }

    // ── Transition to Combat ──────────────────────────────────
    fn start_combat(&mut self) {
        let wave_multiplier = self.wave as i32;
        let enemy = self.spawn_enemy();
        self.enemy = Some(enemy);
        self.turn  = 1;
        self.log.clear();
        self.add_log(format!("⚔  Wave {}! Fight!", self.wave));
        let abs = self.player.as_ref().unwrap().get_available_abilities();
        self.abilities = abs;
        self.cursor    = 0;
        self.screen    = Screen::Combat;
    }

    // ── Player takes an action ────────────────────────────────
    fn player_action(&mut self) {
        let ability = match self.abilities.get(self.cursor) {
            Some(a) => a.clone(),
            None    => return,
        };

        // 1️⃣ Player attacks
        let (dmg, enemy_dead) = {
            let p = self.player.as_mut().unwrap();
            let e = self.enemy.as_mut().unwrap();
            let before = e.current_hp;
            p.use_ability(&ability, e);
            let dmg = (before - e.current_hp).max(0.0);
            (dmg, e.is_dead())
        };
        self.add_log(format!("▶ {} → {:.0} dmg", ability, dmg));

        // 2️⃣ Check victory
        if enemy_dead {
            self.on_victory();
            return;
        }

        // 3️⃣ DoT ticks
        self.tick_dots();
        if self.enemy.as_ref().unwrap().is_dead() {
            self.on_victory();
            return;
        }

        // 4️⃣ Enemy attacks
        self.enemy_attack();
        if self.screen == Screen::GameOver { return; }

        // 5️⃣ End of turn housekeeping
        {
            let p = self.player.as_mut().unwrap();
            let e = self.enemy.as_mut().unwrap();
            p.tick_cooldowns();
            p.regenerate();
            e.regenerate();
        }
        let abs = self.player.as_ref().unwrap().get_available_abilities();
        self.abilities = abs;
        self.turn += 1;
    }

    fn on_victory(&mut self) {
        let (exp, gold, name) = {
            let e = self.enemy.as_ref().unwrap();
            (e.exp_reward, e.gold_reward, e.name.clone())
        };
        let p = self.player.as_mut().unwrap();
        p.gold += gold;
        p.gain_exp(exp);
        self.add_log(format!("✨ {} down! +{} G +{:.0} EXP", name, gold, exp));
        self.screen = Screen::Victory;
    }

    fn tick_dots(&mut self) {
        let e = self.enemy.as_mut().unwrap();
        if e.ignite_turns > 0 {
            let dmg = 35.0;
            e.current_hp = (e.current_hp - dmg).max(0.0);
            e.ignite_turns -= 1;
            self.log.push(format!("🔥 Burn {:.0} dmg ({} left)", dmg, e.ignite_turns));
        }
        if e.poison_stacks > 0 {
            let dmg = e.poison_stacks as f64 * 15.0;
            e.current_hp = (e.current_hp - dmg).max(0.0);
            self.log.push(format!("☠ Poison {:.0} dmg", dmg));
        }
        while self.log.len() > 7 { self.log.remove(0); }
    }

    fn enemy_attack(&mut self) {
        let (frozen, atk, name) = {
            let e = self.enemy.as_ref().unwrap();
            (e.is_frozen, e.base_attack(), e.name.clone())
        };

        if frozen {
            self.enemy.as_mut().unwrap().is_frozen = false;
            self.add_log(format!("🥶 {} is frozen! Skips.", name));
            return;
        }

        let p = self.player.as_mut().unwrap();
        let dmg = (atk - p.stats.armor).max(1.0);
        if p.stats.current_hp <= 0.0 {
            self.screen = Screen::GameOver;
        }
        p.stats.current_hp = (p.stats.current_hp - dmg).max(0.0);
        self.add_log(format!("👾 {} hits for {:.0}!", name, dmg));


    }
}

// ── Pixel Art Sprites ─────────────────────────────────────────

fn draw_warrior(x: f32, y: f32, col: Color) {
    // Helmet
    draw_rectangle(x + 7.0*S, y - 3.0*S, 14.0*S, 5.0*S, DARKGRAY);
    // Head
    draw_rectangle(x + 8.0*S, y + 2.0*S, 12.0*S, 11.0*S, Color::from_rgba(220,180,140,255));
    // Body
    draw_rectangle(x + 6.0*S, y + 13.0*S, 16.0*S, 17.0*S, col);
    // Left arm
    draw_rectangle(x,          y + 13.0*S,  6.0*S, 11.0*S, col);
    // Right arm
    draw_rectangle(x + 22.0*S, y + 13.0*S,  6.0*S, 11.0*S, col);
    // Sword blade
    draw_rectangle(x + 29.0*S, y + 5.0*S,   3.0*S, 22.0*S, LIGHTGRAY);
    // Sword guard
    draw_rectangle(x + 25.0*S, y + 11.0*S,  9.0*S,  3.0*S, LIGHTGRAY);
    // Legs
    draw_rectangle(x + 7.0*S,  y + 30.0*S,  8.0*S, 15.0*S, DARKBLUE);
    draw_rectangle(x + 17.0*S, y + 30.0*S,  8.0*S, 15.0*S, DARKBLUE);
    // Boots
    draw_rectangle(x + 6.0*S,  y + 43.0*S, 10.0*S,  5.0*S, GRAY);
    draw_rectangle(x + 16.0*S, y + 43.0*S, 10.0*S,  5.0*S, GRAY);
}

fn draw_mage(x: f32, y: f32, col: Color) {
    // Hat cone
    draw_triangle(
        vec2(x + 14.0*S, y - 14.0*S),
        vec2(x +  2.0*S, y +  2.0*S),
        vec2(x + 26.0*S, y +  2.0*S),
        col,
    );
    // Hat brim
    draw_rectangle(x + 1.0*S, y + 1.0*S, 26.0*S, 4.0*S, col);
    // Head
    draw_rectangle(x + 8.0*S, y + 5.0*S, 12.0*S, 11.0*S, Color::from_rgba(220,180,140,255));
    // Robe
    draw_rectangle(x + 4.0*S, y + 16.0*S, 20.0*S, 22.0*S, col);
    // Arms
    draw_rectangle(x,          y + 16.0*S,  5.0*S, 12.0*S, col);
    draw_rectangle(x + 23.0*S, y + 16.0*S,  5.0*S, 12.0*S, col);
    // Staff
    draw_rectangle(x + 29.0*S, y - 6.0*S,  3.0*S, 50.0*S, Color::from_rgba(101,67,33,255));
    // Orb
    draw_circle(x + 30.5*S, y - 9.0*S, 5.5*S, Color::from_rgba(160,80,255,255));
    // Feet
    draw_rectangle(x + 5.0*S,  y + 38.0*S,  8.0*S, 7.0*S, DARKGRAY);
    draw_rectangle(x + 15.0*S, y + 38.0*S,  8.0*S, 7.0*S, DARKGRAY);
}

fn draw_rogue(x: f32, y: f32, col: Color) {
    // Hood
    draw_rectangle(x + 5.0*S,  y - 4.0*S, 18.0*S,  8.0*S, DARKGRAY);
    // Head
    draw_rectangle(x + 8.0*S,  y + 2.0*S, 12.0*S, 11.0*S, Color::from_rgba(220,180,140,255));
    // Body (sleek)
    draw_rectangle(x + 7.0*S,  y + 13.0*S,14.0*S, 17.0*S, col);
    // Arms
    draw_rectangle(x +  1.0*S, y + 13.0*S, 6.0*S, 10.0*S, col);
    draw_rectangle(x + 21.0*S, y + 13.0*S, 6.0*S, 10.0*S, col);
    // Left dagger
    draw_rectangle(x -  2.0*S, y +  7.0*S, 3.0*S, 15.0*S, LIGHTGRAY);
    // Right dagger
    draw_rectangle(x + 27.0*S, y +  7.0*S, 3.0*S, 15.0*S, LIGHTGRAY);
    // Legs
    draw_rectangle(x +  7.0*S, y + 30.0*S, 7.0*S, 14.0*S, DARKGRAY);
    draw_rectangle(x + 14.0*S, y + 30.0*S, 7.0*S, 14.0*S, DARKGRAY);
}

fn draw_enemy(x: f32, y: f32, wave: u32) {
    match wave {
        // Goblin
        1 => {
            let col = Color::from_rgba(60, 140, 60, 255);
            draw_rectangle(x + 6.0*S,  y,          16.0*S, 16.0*S, col);
            draw_rectangle(x + 9.0*S,  y + 4.0*S,  4.0*S,  4.0*S, RED);   // eye L
            draw_rectangle(x + 17.0*S, y + 4.0*S,  4.0*S,  4.0*S, RED);   // eye R
            draw_rectangle(x + 8.0*S,  y + 16.0*S,12.0*S, 15.0*S, col);
            // Club
            draw_rectangle(x + 24.0*S, y + 6.0*S,  5.0*S, 20.0*S, Color::from_rgba(101,67,33,255));
            draw_rectangle(x + 22.0*S, y + 6.0*S,  9.0*S,  6.0*S, Color::from_rgba(101,67,33,255));
            draw_rectangle(x + 8.0*S,  y + 31.0*S, 5.0*S, 12.0*S, DARKGREEN);
            draw_rectangle(x + 15.0*S, y + 31.0*S, 5.0*S, 12.0*S, DARKGREEN);
        }
        // Orc
        2 => {
            let col = Color::from_rgba(130, 75, 35, 255);
            draw_rectangle(x +  4.0*S, y,          20.0*S, 18.0*S, col);
            draw_rectangle(x +  7.0*S, y + 5.0*S,  5.0*S,  5.0*S, RED);
            draw_rectangle(x + 16.0*S, y + 5.0*S,  5.0*S,  5.0*S, RED);
            draw_rectangle(x +  2.0*S, y + 18.0*S,24.0*S, 20.0*S, col);
            draw_rectangle(x -  6.0*S, y + 10.0*S, 8.0*S, 22.0*S, col);   // L arm
            draw_rectangle(x + 26.0*S, y + 10.0*S, 8.0*S, 22.0*S, col);   // R arm
            // Axe
            draw_rectangle(x + 27.0*S, y - 2.0*S,  5.0*S, 16.0*S, LIGHTGRAY);
            draw_rectangle(x + 24.0*S, y - 2.0*S, 10.0*S,  8.0*S, LIGHTGRAY);
            draw_rectangle(x +  6.0*S, y + 38.0*S, 8.0*S, 14.0*S, Color::from_rgba(101,67,33,255));
            draw_rectangle(x + 16.0*S, y + 38.0*S, 8.0*S, 14.0*S, Color::from_rgba(101,67,33,255));
        }
        // Dark Knight
        3 => {
            let col = Color::from_rgba(50, 50, 70, 255);
            draw_rectangle(x +  5.0*S, y - 7.0*S, 18.0*S,  9.0*S, DARKGRAY); // helmet
            draw_rectangle(x +  5.0*S, y + 2.0*S, 18.0*S, 16.0*S, col);
            draw_rectangle(x +  8.0*S, y + 6.0*S,  4.0*S,  4.0*S, RED);
            draw_rectangle(x + 16.0*S, y + 6.0*S,  4.0*S,  4.0*S, RED);
            draw_rectangle(x +  3.0*S, y + 18.0*S,22.0*S, 22.0*S, DARKGRAY);
            draw_rectangle(x -  8.0*S, y + 12.0*S,11.0*S, 14.0*S, DARKGRAY); // L arm
            draw_rectangle(x + 25.0*S, y + 12.0*S,11.0*S, 14.0*S, DARKGRAY); // R arm
            // Longsword
            draw_rectangle(x + 28.0*S, y - 10.0*S, 4.0*S, 50.0*S, Color::from_rgba(180,20,20,255));
            draw_rectangle(x + 23.0*S, y +  4.0*S,14.0*S,  4.0*S, GRAY);    // guard
            draw_rectangle(x +  6.0*S, y + 40.0*S,10.0*S, 13.0*S, DARKGRAY);
            draw_rectangle(x + 18.0*S, y + 40.0*S,10.0*S, 13.0*S, DARKGRAY);
        }
        // Dragon
        4 => {
            let col = Color::from_rgba(180, 50, 15, 255);
            // Wings
            draw_triangle(vec2(x - 20.0*S, y + 5.0*S),  vec2(x, y + 10.0*S), vec2(x, y + 35.0*S), Color::from_rgba(180,50,15,160));
            draw_triangle(vec2(x + 60.0*S, y + 5.0*S),  vec2(x + 40.0*S, y + 10.0*S), vec2(x + 40.0*S, y + 35.0*S), Color::from_rgba(180,50,15,160));
            // Body
            draw_rectangle(x,           y + 10.0*S, 40.0*S, 24.0*S, col);
            // Head
            draw_rectangle(x +  7.0*S, y,           24.0*S, 12.0*S, col);
            draw_rectangle(x + 10.0*S, y + 4.0*S,   4.0*S,  5.0*S, YELLOW);
            draw_rectangle(x + 24.0*S, y + 4.0*S,   4.0*S,  5.0*S, YELLOW);
            // Tail
            draw_rectangle(x - 10.0*S, y + 28.0*S, 12.0*S, 6.0*S, col);
            draw_rectangle(x - 18.0*S, y + 30.0*S,  9.0*S, 4.0*S, col);
            draw_rectangle(x - 24.0*S, y + 32.0*S,  7.0*S, 3.0*S, col);
            // Legs
            draw_rectangle(x +  5.0*S, y + 34.0*S,  8.0*S, 14.0*S, col);
            draw_rectangle(x + 26.0*S, y + 34.0*S,  8.0*S, 14.0*S, col);
        }
        // Ancient Golem
        _ => {
            let col = Color::from_rgba(110, 110, 130, 255);
            // Body (massive)
            draw_rectangle(x,           y + 5.0*S,  40.0*S, 35.0*S, col);
            // Head (square)
            draw_rectangle(x +  6.0*S, y - 4.0*S,  26.0*S, 12.0*S, DARKGRAY);
            draw_rectangle(x + 10.0*S, y +  0.0*S,  6.0*S,  4.0*S, Color::from_rgba(0,200,255,255));
            draw_rectangle(x + 22.0*S, y +  0.0*S,  6.0*S,  4.0*S, Color::from_rgba(0,200,255,255));
            // Arms (huge)
            draw_rectangle(x - 12.0*S, y + 5.0*S,  12.0*S, 30.0*S, col);
            draw_rectangle(x + 40.0*S, y + 5.0*S,  12.0*S, 30.0*S, col);
            // Fists
            draw_rectangle(x - 14.0*S, y + 33.0*S, 16.0*S, 12.0*S, DARKGRAY);
            draw_rectangle(x + 38.0*S, y + 33.0*S, 16.0*S, 12.0*S, DARKGRAY);
            // Legs
            draw_rectangle(x +  4.0*S, y + 40.0*S, 12.0*S, 16.0*S, col);
            draw_rectangle(x + 22.0*S, y + 40.0*S, 12.0*S, 16.0*S, col);
        }
    }
}

// ── UI helpers ────────────────────────────────────────────────

fn panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(8, 8, 18, 215));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::from_rgba(70, 70, 110, 220));
}

fn hp_bar(x: f32, y: f32, w: f32, h: f32, pct: f32, col: Color) {
    draw_rectangle(x, y, w, h, Color::from_rgba(30, 8, 8, 200));
    draw_rectangle(x + 1.0, y + 1.0, (w - 2.0) * pct.clamp(0.0, 1.0), h - 2.0, col);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(180, 180, 180, 80));
}

// ── Screen draws ─────────────────────────────────────────────

fn draw_main_menu() {
    let t = get_time() as f32;
    // Stars
    for i in 0u32..60 {
        let bx = (i as f32 * 137.5) % W;
        let by = (i as f32 *  93.3) % H;
        let r  = 0.8 + ((t * 1.5 + i as f32 * 0.4).sin().abs()) * 1.4;
        draw_circle(bx, by, r, Color::from_rgba(200,200,255,120));
    }
    // Title
    draw_text("⚔  KOKTEL TALES  ⚔", W/2.0 - 160.0, H/2.0 - 55.0, 38.0, GOLD);
    // Pulse
    let alpha = ((t * 2.2).sin() * 0.4 + 0.6) as f32;
    draw_text("Press  ENTER  to Begin",
        W/2.0 - 100.0, H/2.0 + 15.0, 22.0,
        Color::from_rgba(210, 210, 210, (alpha * 255.0) as u8));
    draw_text("[Q] Quit", W/2.0 - 35.0, H/2.0 + 55.0, 15.0, GRAY);
}

fn draw_class_select(cursor: usize) {
    panel(50.0, 30.0, W - 100.0, H - 60.0);
    draw_text("SELECT CLASS", W/2.0 - 80.0, 68.0, 26.0, GOLD);
    draw_text("↑ / ↓  move      ENTER  confirm", W/2.0 - 130.0, 95.0, 14.0, GRAY);

    let opts = class_options();
    for (i, opt) in opts.iter().enumerate() {
        let y   = 128.0 + i as f32 * 52.0;
        let sel = i == cursor;
        if sel {
            draw_rectangle(60.0, y - 6.0, W - 120.0, 44.0, Color::from_rgba(40,40,70,200));
            draw_rectangle_lines(60.0, y - 6.0, W - 120.0, 44.0, 1.5, opt.color);
        }
        // Color swatch
        draw_rectangle(70.0, y + 4.0, 14.0, 28.0, opt.color);
        let txt_col = if sel { opt.color } else { Color::from_rgba(150,150,150,255) };
        draw_text(opt.label, 94.0, y + 24.0, 18.0, txt_col);
    }
}

fn draw_combat_screen(state: &GameState) {
    let p = match state.player.as_ref() { Some(x) => x, None => return };
    let t = get_time() as f32;

    // Background: Sky gradient and Ground
    draw_rectangle(0.0, 0.0, W, GROUND_Y, Color::from_rgba(12, 12, 22, 255));
    draw_rectangle(0.0, GROUND_Y, W, H - GROUND_Y, Color::from_rgba(20, 20, 35, 255));
    draw_line(0.0, GROUND_Y, W, GROUND_Y, 1.5, Color::from_rgba(55, 55, 95, 220));

    // Boss Warning UI
    let waves_until_boss = if state.wave % 5 == 0 { 0 } else { 5 - (state.wave % 5) };
    if waves_until_boss == 0 {
        draw_text("!!! BOSS FIGHT IN PROGRESS !!!", W / 2.0 - 150.0, 40.0, 24.0, RED);
    } else if waves_until_boss == 1 {
        draw_text("WARNING: BOSS APPROACHES NEXT WAVE!", W / 2.0 - 180.0, 40.0, 20.0, ORANGE);
    } else {
        draw_text(&format!("Waves until next Boss: {}", waves_until_boss), W / 2.0 - 110.0, 40.0, 16.0, LIGHTGRAY);
    }

    // Player Sprite Rendering
    let p_col = match p.class {
        PlayerClass::Warrior => Color::from_rgba(60, 100, 200, 255),
        PlayerClass::Rogue   => Color::from_rgba(100, 60, 160, 255),
        PlayerClass::Mage    => Color::from_rgba(60, 160, 200, 255),
    };
    let bob = 1.5 * (t * 2.4).sin().abs();
    let py = GROUND_Y - 48.0 * S - bob;
    
    match p.class {
        PlayerClass::Warrior => draw_warrior(100.0, py, p_col),
        PlayerClass::Mage    => draw_mage(100.0, py, p_col),
        PlayerClass::Rogue   => draw_rogue(100.0, py, p_col),
    }

    // Player HP/MP Bars & Stickers
    let hp_pct = (p.stats.current_hp / p.stats.max_hp) as f32;
    let mp_pct = (p.stats.current_mana / p.stats.max_mana) as f32;
    hp_bar(90.0, py - 30.0, 110.0, 10.0, hp_pct, GREEN);
    hp_bar(90.0, py - 16.0, 110.0, 8.0, mp_pct, Color::from_rgba(50, 80, 220, 255));
    draw_text(&format!("{:.0}/{:.0}", p.stats.current_hp, p.stats.max_hp), 205.0, py - 22.0, 10.0, LIGHTGRAY);
    
    let p_sticker = match p.class {
        PlayerClass::Warrior => "[O_O]",
        PlayerClass::Rogue   => "(-_-)",
        PlayerClass::Mage    => "(^.^)",
    };
    draw_text(&format!("{} Hero", p_sticker), 90.0, py - 38.0, 16.0, WHITE);

    // Enemy Sprite Rendering & UI
    if let Some(ref e) = state.enemy {
        let is_boss = state.wave % 5 == 0;
        let ex = 540.0;
        let ey = GROUND_Y - 50.0 * S;
        
        // Pass sprite ID 5 to render the massive Ancient Golem for Bosses
        let sprite_id = if is_boss { 5 } else { state.wave % 5 };
        draw_enemy(ex, ey, sprite_id);

        let en_pct = (e.current_hp / e.max_hp) as f32;
        hp_bar(ex - 5.0, ey - 28.0, 130.0, 10.0, en_pct, RED);
        draw_text(&format!("{:.0}/{:.0}", e.current_hp, e.max_hp), ex + 130.0, ey - 22.0, 10.0, LIGHTGRAY);

        let e_sticker = if is_boss { "\\m/ (X_X) \\m/" } else { "(>_<)" };
        let title_col = if is_boss { RED } else { WHITE };
        draw_text(&format!("{} {}", e_sticker, e.name), ex - 5.0, ey - 40.0, 16.0, title_col);

        // Status Icons
        if e.is_frozen { draw_text("FROZEN", ex - 5.0, ey - 56.0, 11.0, Color::from_rgba(120, 210, 255, 255)); }
        if e.ignite_turns > 0 { draw_text(&format!("BURN x{}", e.ignite_turns), ex + 60.0, ey - 56.0, 11.0, ORANGE); }
        if e.poison_stacks > 0 { draw_text(&format!("POISON x{}", e.poison_stacks), ex + 120.0, ey - 56.0, 11.0, GREEN); }
    }

    // Bottom UI Panels
    let ui_y = GROUND_Y + 8.0;
    let ui_h = H - ui_y - 8.0;

    // Stats Panel
    panel(8.0, ui_y, 215.0, ui_h);
    draw_text(&format!("Lv{}  Turn {}  Wave {}", p.level, state.turn, state.wave), 16.0, ui_y + 18.0, 12.0, GOLD);
    draw_text(&format!("HP  {:.0}/{:.0}", p.stats.current_hp, p.stats.max_hp), 16.0, ui_y + 34.0, 12.0, GREEN);
    draw_text(&format!("MP  {:.0}/{:.0}", p.stats.current_mana, p.stats.max_mana), 16.0, ui_y + 50.0, 12.0, Color::from_rgba(80, 120, 255, 255));
    draw_text(&format!("STR {:.0}  AGI {:.0}  INT {:.0}", p.stats.strength, p.stats.agility, p.stats.intelligence), 16.0, ui_y + 66.0, 11.0, YELLOW);
    draw_text(&format!("Armor {:.1}   Crit {:.0}%", p.stats.armor, p.stats.crit_chance * 100.0), 16.0, ui_y + 82.0, 11.0, LIGHTGRAY);
    draw_text(&format!("Gold {}   TP {}", p.gold, p.talent_points), 16.0, ui_y + 98.0, 11.0, GOLD);
    draw_text(&format!("EXP {:.0}/{:.0}", p.exp, p.exp_to_next_level()), 16.0, ui_y + 114.0, 11.0, LIGHTGRAY);
    draw_text(&format!("Regen HP+{:.1}  MP+{:.1}", p.stats.hp_regen, p.stats.mana_regen), 16.0, ui_y + 130.0, 11.0, DARKGRAY);

    // Ability Panel
    panel(230.0, ui_y, 315.0, ui_h);
    draw_text("ABILITIES", 240.0, ui_y + 18.0, 12.0, GRAY);
    for (i, ab) in state.abilities.iter().enumerate() {
        let ay = ui_y + 32.0 + i as f32 * 20.0;
        let sel = i == state.cursor;
        if sel {
            draw_rectangle(233.0, ay - 13.0, 309.0, 18.0, Color::from_rgba(50, 50, 90, 200));
        }
        let cd = p.cooldowns.get(ab).copied().unwrap_or(0);
        let col = if sel { WHITE } else { Color::from_rgba(145, 145, 145, 255) };
        let cd_s = if cd > 0 { format!(" [{}]", cd) } else { String::new() };
        draw_text(&format!("[{}] {}{}", i + 1, ab, cd_s), 242.0, ay, 12.0, col);
    }
    draw_text("Arrows/1-9 select    ENTER use    Q quit", 240.0, H - 12.0, 10.0, DARKGRAY);

    // Log Panel
    panel(552.0, ui_y, W - 560.0, ui_h);
    draw_text("LOG", 562.0, ui_y + 18.0, 12.0, GRAY);
    for (i, line) in state.log.iter().enumerate() {
        draw_text(line, 562.0, ui_y + 32.0 + i as f32 * 20.0, 11.0, WHITE);
    }
}

fn draw_victory(state: &GameState) {
    let t = get_time() as f32;
    draw_rectangle(0.0, 0.0, W, H, Color::from_rgba(5, 20, 5, 235));

    let col = Color::from_rgba(80, 255, 80, 255);
    draw_text("VICTORY!", W/2.0 - 85.0, H/2.0 - 70.0, 44.0, col);
    draw_text(&format!("Wave {} complete!", state.wave), W/2.0 - 90.0, H/2.0 - 15.0, 24.0, GOLD);

    if let Some(ref p) = state.player {
        draw_text(&format!("Level {}   Gold {}   EXP {:.0}",
            p.level, p.gold, p.exp),
            W/2.0 - 130.0, H/2.0 + 25.0, 18.0, LIGHTGRAY);
    }

    let a = ((t * 2.0).sin() * 0.4 + 0.6) as f32;
    draw_text("ENTER — Next Wave     Q — Quit",
        W/2.0 - 135.0, H/2.0 + 80.0, 18.0,
        Color::from_rgba(200,200,200,(a*255.0) as u8));
}

fn draw_game_over(state: &GameState) {
    let t = get_time() as f32;
    draw_rectangle(0.0, 0.0, W, H, Color::from_rgba(18, 3, 3, 240));

    draw_text("YOU  DIED", W/2.0 - 110.0, H/2.0 - 70.0, 46.0,
        Color::from_rgba(200, 20, 20, 255));

    if let Some(ref p) = state.player {
        draw_text(&format!("Wave {}   Gold {}   Level {}", state.wave, p.gold, p.level),
            W/2.0 - 115.0, H/2.0, 20.0, GRAY);
    }

    let a = ((t * 2.0).sin() * 0.4 + 0.6) as f32;
    draw_text("ENTER — New Game     Q — Quit",
        W/2.0 - 130.0, H/2.0 + 65.0, 18.0,
        Color::from_rgba(200,200,200,(a*255.0) as u8));
}

// ── Main loop ─────────────────────────────────────────────────
#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::new();

    loop {
        clear_background(Color::from_rgba(12, 12, 22, 255));

        match state.screen.clone() {

            Screen::MainMenu => {
                draw_main_menu();
                if is_key_pressed(KeyCode::Enter) { state.screen = Screen::ClassSelect; }
                if is_key_pressed(KeyCode::Q) { break; }
            }

            Screen::ClassSelect => {
                draw_class_select(state.cursor);
                let max = class_options().len() - 1;
                if is_key_pressed(KeyCode::Up)   && state.cursor > 0   { state.cursor -= 1; }
                if is_key_pressed(KeyCode::Down)  && state.cursor < max { state.cursor += 1; }
                if is_key_pressed(KeyCode::Enter) {
                    let opts = class_options();
                    let p    = GameState::build_player(&opts[state.cursor]);
                    state.player = Some(p);
                    state.wave   = 1;
                    state.cursor = 0;
                    state.start_combat();
                }
                if is_key_pressed(KeyCode::Q) { break; }
            }

            Screen::Combat => {
                draw_combat_screen(&state);

                let ab_len = state.abilities.len();

                // ↑ ↓ navigate abilities
                if is_key_pressed(KeyCode::Up) && state.cursor > 0 {
                    state.cursor -= 1;
                }
                if is_key_pressed(KeyCode::Down) && state.cursor < ab_len.saturating_sub(1) {
                    state.cursor += 1;
                }

                // Number hotkeys 1–9
                let hotkeys = [
                    KeyCode::Key1, KeyCode::Key2, KeyCode::Key3,
                    KeyCode::Key4, KeyCode::Key5, KeyCode::Key6,
                    KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
                ];
                for (i, &k) in hotkeys.iter().enumerate() {
                    if is_key_pressed(k) && i < ab_len {
                        state.cursor = i;
                        state.player_action();
                        break;
                    }
                }

                // Enter to confirm selected ability
                if is_key_pressed(KeyCode::Enter) {
                    state.player_action();
                }

                if is_key_pressed(KeyCode::Q) { break; }
            }

            Screen::Victory => {
                draw_victory(&state);
                if is_key_pressed(KeyCode::Enter) {
                state.cursor = 0;
                state.screen = Screen::LevelUp; // Bridge to LevelUp screen instead of starting combat
        }
            if is_key_pressed(KeyCode::Q) {
                break;
    }
            }
            Screen::LevelUp => {
                let player = state.player.as_ref().unwrap();
                let options: Vec<(&'static str, &'static str)> =mechanics::get_talent_list(player);
                let exit_idx = options.len();

    // Draw background overlay panel
    draw_rectangle(100.0, 100.0, 600.0, 400.0, Color::from_rgba(20, 20, 35, 240));
    draw_text("LEVEL UP - SPELL ARCHIVES", 140.0, 150.0, 24.0, GOLD);
    draw_text(&format!("Available Talent Points (TP): {}", player.talent_points), 140.0, 180.0, 16.0, WHITE);
    draw_line(140.0, 195.0, 660.0, 195.0, 2.0, MAROON);

    // Render talent choices dynamically
    let mut y_pos = 240.0;
    for (i, (name, desc)) in options.iter().enumerate() {
        let is_selected = state.cursor == i;
        let txt_color = if is_selected { YELLOW } else { GRAY };
        let prefix = if is_selected { " > [LEARN] " } else { "   " };
        
        draw_text(&format!("{}{}", prefix, name.to_uppercase()), 150.0, y_pos, 18.0, txt_color);
        draw_text(desc, 300.0, y_pos, 14.0, if is_selected { LIGHTGRAY } else { DARKGRAY });
        y_pos += 40.0;
    }

    // Render exit option to move to the shop
    let is_exit_selected = state.cursor == exit_idx;
    let exit_color = if is_exit_selected { GREEN } else { GRAY };
    let exit_prefix = if is_exit_selected { " > " } else { "   " };
    draw_text(&format!("{}[ PROCEED TO MERCHANT SHOP ]", exit_prefix), 150.0, y_pos + 20.0, 18.0, exit_color);

    // Input Controller for Navigation
    if is_key_pressed(KeyCode::Up) && state.cursor > 0 {
        state.cursor -= 1;
    }
    if is_key_pressed(KeyCode::Down) && state.cursor < exit_idx {
        state.cursor += 1;
    }
    if is_key_pressed(KeyCode::Enter) {
        if state.cursor == exit_idx {
            // Generate customized random shop stock based on Player class and current Wave
            state.shop = Some(crate::shop::Shop::generate_shop_stock(state.player.as_ref().unwrap(), state.wave));
            state.cursor = 0;
            state.screen = Screen::Shop;
        } else if player.talent_points > 0 {
            let selected_talent = options[state.cursor].0;
            let p = state.player.as_mut().unwrap();
            if p.upgrade_generic_talent(selected_talent) {
                state.add_log(format!("Successfully upgraded spell: {}", selected_talent));
            }
        }
    }
}

Screen::Shop => {
    let player = state.player.as_ref().unwrap();
    
    // Draw background overlay panel
    draw_rectangle(100.0, 100.0, 600.0, 400.0, Color::from_rgba(30, 25, 20, 240));
    draw_text("WANDERING MERCHANT SHOP", 140.0, 150.0, 24.0, GOLD);
    draw_text(&format!("Your Balance: {} G", player.gold), 140.0, 180.0, 16.0, YELLOW);
    draw_line(140.0, 195.0, 660.0, 195.0, 2.0, ORANGE);

    let mut shop_items_len = 0;
    let mut y_pos = 240.0;

    if let Some(ref s) = state.shop {
        shop_items_len = s.items_for_sale.len();
        for (i, item) in s.items_for_sale.iter().enumerate() {
            let is_selected = state.cursor == i;
            let txt_color = if is_selected { YELLOW } else { WHITE };
            let prefix = if is_selected { " > [BUY] " } else { "   " };
            
            draw_text(&format!("{}{}", prefix, item.name), 150.0, y_pos, 16.0, txt_color);
            draw_text(&format!("Cost: {} G", item.price), 520.0, y_pos, 16.0, if is_selected { GOLD } else { LIGHTGRAY });
            y_pos += 40.0;
        }
    }

    let is_exit_selected = state.cursor == shop_items_len;
    let exit_color = if is_exit_selected { RED } else { GRAY };
    let exit_prefix = if is_exit_selected { " > " } else { "   " };
    draw_text(&format!("{}[ VENTURE INTO WAVE {} ]", exit_prefix, state.wave + 1), 150.0, y_pos + 20.0, 18.0, exit_color);

    
    if is_key_pressed(KeyCode::Up) && state.cursor > 0 {
        state.cursor -= 1;
    }
    if is_key_pressed(KeyCode::Down) && state.cursor < shop_items_len {
        state.cursor += 1;
    }
    if is_key_pressed(KeyCode::Enter) {
        if state.cursor == shop_items_len {
            
            state.wave += 1;
            state.cursor = 0;
            if let Some(ref mut p) = state.player {
                p.stats.current_hp = p.stats.max_hp;
                p.stats.current_mana = p.stats.max_mana;
                p.cooldowns.clear(); 
            }
            state.start_combat();
    } else if state.shop.is_some() {
    // Temporarily take the shop out of state to bypass borrow checker restrictions
        let mut current_shop = state.shop.take().unwrap();
    
        if let Some(ref mut p) = state.player {
            let before_len = current_shop.items_for_sale.len();
        
        
            current_shop.buy_item(p, state.cursor);
        
        
        if current_shop.items_for_sale.len() < before_len {
            state.add_log("Item purchased and applied to stats!");
            if state.cursor >= current_shop.items_for_sale.len() && state.cursor > 0 {
                state.cursor -= 1;
            }
        } else {
            state.add_log("Transaction failed: Insufficient gold!");
        }
    }
    
    
    state.shop = Some(current_shop);
        }
    }
}

            Screen::GameOver => {
                draw_game_over(&state);
                if is_key_pressed(KeyCode::Enter) { state = GameState::new(); }
                if is_key_pressed(KeyCode::Q) { break; }
            }
        }

        next_frame().await;
    }
}

// ── Tests (cargo test — no macroquad needed) ──────────────────
#[cfg(test)]
mod tests {
    use crate::mechanics::{Player, PlayerClass, PlayerSpec, mana_cost};
    use crate::combat::Enemy;
    use crate::talents::PlayerTalentTree;

    fn dummy(hp: f64) -> Enemy { Enemy::new("Dummy", hp, 0.0, 0.0, 0) }

    #[test] fn test_warrior_dps_base_stats() {
        let p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        assert_eq!(p.stats.strength, 30.0);
        assert_eq!(p.stats.agility,  18.0);
        assert_eq!(p.stats.intelligence, 15.0);
        assert_eq!(p.level, 1);
        assert_eq!(p.talent_points, 0);
        assert_eq!(p.gold, 0);
    }

    #[test] fn test_warrior_tank_base_stats() {
        let p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorTank);
        assert_eq!(p.stats.strength, 32.0);
        assert_eq!(p.stats.agility,  15.0);
        assert!((p.stats.armor - 15.0 * 0.65).abs() < 0.1);
    }

    #[test] fn test_tank_has_more_armor_than_dps() {
        let tank = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorTank);
        let dps  = Player::new("D", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        assert!(tank.stats.armor > dps.stats.armor);
    }

    #[test] fn test_mana_costs_defined() {
        assert_eq!(mana_cost("basic_attack"), 0.0);
        assert_eq!(mana_cost("frostbolt"),   20.0);
        assert_eq!(mana_cost("fireball"),    25.0);
        assert_eq!(mana_cost("void_bolt"),   18.0);
        assert_eq!(mana_cost("holy_heal"),   30.0);
        assert_eq!(mana_cost("shadow_step"), 15.0);
    }

    #[test] fn test_frostbolt_deducts_mana() {
        let mut m = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        m.choose_elemental_spec("ice");
        m.talent_points = 1;
        m.upgrade_mage_dps_talent("frostbolt");
        let before = m.stats.current_mana;
        m.use_ability("frostbolt", &mut dummy(1000.0));
        assert_eq!(m.stats.current_mana, before - mana_cost("frostbolt"));
    }

    #[test] fn test_not_enough_mana_blocks_cast() {
        let mut m = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        m.choose_elemental_spec("ice");
        m.talent_points = 1;
        m.upgrade_mage_dps_talent("frostbolt");
        m.stats.current_mana = 5.0;
        let mut e = dummy(1000.0);
        m.use_ability("frostbolt", &mut e);
        assert_eq!(m.stats.current_mana, 5.0);
        assert_eq!(e.current_hp, 1000.0);
    }

    #[test] fn test_basic_attack_costs_no_mana() {
        let mut w = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let before = w.stats.current_mana;
        w.use_ability("basic_attack", &mut dummy(500.0));
        assert_eq!(w.stats.current_mana, before);
    }

    #[test] fn test_basic_attack_deals_damage() {
        let mut w = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let mut e = dummy(500.0);
        w.use_ability("basic_attack", &mut e);
        assert!(e.current_hp < 500.0);
    }

    #[test] fn test_ice_lance_bonus_on_frozen() {
        let mut m = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        m.choose_elemental_spec("ice");
        m.talent_points = 2;
        m.upgrade_mage_dps_talent("frostbolt");
        m.upgrade_mage_dps_talent("ice_lance");
        m.stats.current_mana = 999.0;

        let mut e_normal = dummy(1000.0);
        m.use_ability("ice_lance", &mut e_normal);
        let dmg_n = 1000.0 - e_normal.current_hp;

        m.tick_cooldowns();

        let mut e_frozen = dummy(1000.0);
        e_frozen.is_frozen = true;
        m.stats.current_mana = 999.0;
        m.use_ability("ice_lance", &mut e_frozen);
        let dmg_f = 1000.0 - e_frozen.current_hp;

        assert!(dmg_f > dmg_n, "Frozen({:.1}) should > Normal({:.1})", dmg_f, dmg_n);
    }

    #[test] fn test_enemy_gold_and_exp_reward() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let mut e = Enemy::new("G", 1.0, 0.0, 75.0, 50);
        e.current_hp = 0.0;
        p.defeat_enemy(&e);
        assert_eq!(p.gold, 50);
    }

    #[test] fn test_level_up_grants_talent_point() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        p.gain_exp(9999.0);
        assert!(p.talent_points >= 1);
    }

    #[test] fn test_level_up_increases_stats() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        let hp0 = p.stats.max_hp;
        let st0 = p.stats.strength;
        p.level_up();
        assert!(p.stats.max_hp   > hp0);
        assert!(p.stats.strength > st0);
    }

    #[test] fn test_multi_level_up_loop() {
        let mut p = Player::new("T", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
        p.gain_exp(9999.0);
        assert!(p.level > 3);
        assert_eq!(p.talent_points, p.level - 1);
    }

    #[test] fn test_frostbolt_cooldown() {
        let mut m = Player::new("J", PlayerClass::Mage, PlayerSpec::MageElemental);
        m.choose_elemental_spec("ice");
        m.talent_points = 1;
        m.upgrade_mage_dps_talent("frostbolt");
        m.stats.current_mana = 999.0;
        let mut e = dummy(1000.0);

        m.use_ability("frostbolt", &mut e);             // cd = 3
        let after1 = m.stats.current_mana;
        m.use_ability("frostbolt", &mut e);             // blocked
        assert_eq!(m.stats.current_mana, after1);

        m.tick_cooldowns();
        m.tick_cooldowns();
        m.tick_cooldowns();                             // cd → 0

        m.use_ability("frostbolt", &mut e);             // fires again
        assert!(m.stats.current_mana < after1);
    }

    #[test] fn test_rogue_flurry_deals_more_than_single_hit() {
        let mut rf = Player::new("S", PlayerClass::Rogue, PlayerSpec::RogueAssassin);
        let mut rn = Player::new("S", PlayerClass::Rogue, PlayerSpec::RogueAssassin);
        rn.stats.agility = 0.0;
        if let PlayerTalentTree::Rogue(ref mut r) = rf.talents { r.flurry_blades_lvl = 1; }

        let mut ea = dummy(5000.0);
        let mut eb = dummy(5000.0);
        for _ in 0..10 {
            rf.use_ability("basic_attack", &mut ea);
            rn.use_ability("basic_attack", &mut eb);
        }
        assert!(5000.0 - ea.current_hp > 5000.0 - eb.current_hp);
    }
}
