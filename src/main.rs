fn main() {
    let start_profile = cstreak::Profile {
        level: 37,
        xp: 4114,
    };

    let current_profile = cstreak::Profile { level: 38, xp: 230 };

    let earned_xp = start_profile.earned_xp(&current_profile);

    println!("XP-Status: {:?}/{}", earned_xp, cstreak::total_target());

    for variance in [1.00, 0.95, 1.05] {
        let updated_score = (600.0 * variance) as i64;
        let games = earned_xp.expected_games(cstreak::Game::Deathmatch {
            score: updated_score,
        });

        let duration = std::time::Duration::from_secs(games as u64 * 10 * 60);

        println!(
            "Needed ({:.0}% - score {})",
            (variance - 1.0) * 100.0,
            updated_score
        );
        println!("Games: {}", games);
        println!("Duration: {:?}", duration);
    }
}
