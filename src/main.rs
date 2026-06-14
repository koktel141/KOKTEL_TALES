mod mechanics;
mod talents;
mod combat;

use mechanics::{Player, PlayerClass, PlayerSpec, mana_cost};
use combat::Enemy;
use talents::PlayerTalentTree;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║           RPG GAME — DEMO                ║");
    println!("╚══════════════════════════════════════════╝\n");

    // ── Warrior DPS ─────────────────────────────────────────
    println!("━━━ WARRIOR DPS TEST ━━━");
    let mut warrior = Player::new("Thor", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
    warrior.print_stats();

    let mut goblin = Enemy::new("Goblin Scout", 80.0, 5.0, 75.0, 30);
    println!("\n⚔️  Combat start: {} vs {}", warrior.name, goblin.name);
    while !goblin.is_dead() {
        warrior.use_ability("basic_attack", &mut goblin);
        if goblin.is_dead() {
            warrior.defeat_enemy(&goblin);
        }
    }
    warrior.print_stats();

    // ── Warrior Tank ─────────────────────────────────────────
    println!("\n━━━ WARRIOR TANK TEST ━━━");
    let mut tank = Player::new("Shield", PlayerClass::Warrior, PlayerSpec::WarriorTank);
    tank.print_stats();
    println!(
        "Tank armor: {:.2} vs DPS armor: {:.2}",
        tank.stats.armor,
        warrior.stats.armor
    );

    // ── Rogue Assassin ───────────────────────────────────────
    println!("\n━━━ ROGUE ASSASSIN TEST ━━━");
    let mut rogue = Player::new("Shadow", PlayerClass::Rogue, PlayerSpec::RogueAssassin);
    if let PlayerTalentTree::Rogue(ref mut r) = rogue.talents {
        r.flurry_blades_lvl = 1;
    }
    rogue.print_stats();

    let mut orc = Enemy::new("Orc Grunt", 200.0, 5.0, 100.0, 50);
    println!("\n⚔️  Rogue attacks with Flurry Blades!");
    rogue.use_ability("basic_attack", &mut orc);
    println!("Orc HP after flurry: {:.1}", orc.current_hp);

    // ── Mage Ice ─────────────────────────────────────────────
    println!("\n━━━ MAGE ICE TEST ━━━");
    let mut mage = Player::new("Jaina", PlayerClass::Mage, PlayerSpec::MageElemental);
    mage.choose_elemental_spec("ice");
    mage.talent_points = 3;
    mage.upgrade_mage_dps_talent("frostbolt");
    mage.upgrade_mage_dps_talent("ice_lance");
    mage.print_stats();

    let mut troll = Enemy::new("Ice Troll", 300.0, 8.0, 120.0, 60);
    println!("\n🧙 Mage combo: Frostbolt → Ice Lance (shatter)");
    mage.use_ability("frostbolt", &mut troll);   // freeze
    mage.tick_cooldowns();
    mage.use_ability("ice_lance", &mut troll);    // shatter x2
    println!("Troll HP: {:.1}/{:.1}", troll.current_hp, troll.max_hp);

    // ── Mage Fire ────────────────────────────────────────────
    println!("\n━━━ MAGE FIRE TEST ━━━");
    let mut fire_mage = Player::new("Kael", PlayerClass::Mage, PlayerSpec::MageElemental);
    fire_mage.choose_elemental_spec("fire");
    fire_mage.talent_points = 2;
    fire_mage.upgrade_mage_dps_talent("fireball");
    fire_mage.upgrade_mage_dps_talent("ignite");

    let mut wolf = Enemy::new("Fire Wolf", 150.0, 3.0, 80.0, 40);
    println!("\n🔥 Fire Mage attacks!");
    fire_mage.use_ability("fireball", &mut wolf);
    println!("Wolf HP: {:.1}/{:.1}", wolf.current_hp, wolf.max_hp);

    // ── Mana starved test ────────────────────────────────────
    println!("\n━━━ MANA SYSTEM TEST ━━━");
    let mut broke_mage = Player::new("Oom", PlayerClass::Mage, PlayerSpec::MageElemental);
    broke_mage.choose_elemental_spec("ice");
    broke_mage.talent_points = 1;
    broke_mage.upgrade_mage_dps_talent("frostbolt");
    broke_mage.stats.current_mana = 5.0;

    let mut dummy = Enemy::new("Dummy", 999.0, 0.0, 0.0, 0);
    println!("Mana: {:.0} — trying to cast Frostbolt (costs 20):", broke_mage.stats.current_mana);
    broke_mage.use_ability("frostbolt", &mut dummy);
    println!("Dummy HP unchanged: {:.0}", dummy.current_hp);

    // ── Level Up chain ───────────────────────────────────────
    println!("\n━━━ LEVEL UP TEST ━━━");
    let mut hero = Player::new("Hero", PlayerClass::Warrior, PlayerSpec::WarriorDPS);
    println!("Before: Lv{} STR:{:.0} HP:{:.0}", hero.level, hero.stats.strength, hero.stats.max_hp);
    hero.gain_exp(9999.0);
    println!("After:  Lv{} STR:{:.0} HP:{:.0} TP:{}", hero.level, hero.stats.strength, hero.stats.max_hp, hero.talent_points);

    // ── Cooldown visual ──────────────────────────────────────
    println!("\n━━━ COOLDOWN TEST ━━━");
    let mut cd_mage = Player::new("CD", PlayerClass::Mage, PlayerSpec::MageElemental);
    cd_mage.choose_elemental_spec("ice");
    cd_mage.talent_points = 1;
    cd_mage.upgrade_mage_dps_talent("frostbolt");
    cd_mage.stats.current_mana = 999.0;

    let mut target = Enemy::new("Target", 1000.0, 0.0, 0.0, 0);
    println!("Turn 1 — cast:");
    cd_mage.use_ability("frostbolt", &mut target);
    println!("Turn 2 — on cooldown:");
    cd_mage.use_ability("frostbolt", &mut target);
    println!("tick...");
    cd_mage.tick_cooldowns();
    println!("Turn 3 — ready again:");
    cd_mage.use_ability("frostbolt", &mut target);
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