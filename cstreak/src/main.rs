fn main() {
    let start_profile = cstreak::Profile {
        level: 37,
        xp: 4114,
    };

    let current_profile = cstreak::Profile { level: 1, xp: 160 };

    let earned_xp = start_profile.earned_xp(&current_profile);

    println!(
        "XP-Status: {:?}/{} ({:.0}%)",
        earned_xp,
        cstreak::total_target(),
        earned_xp.0 as f32 * 100.0 / cstreak::total_target() as f32
    );

    for variance in [1.00, 0.90, 1.10] {
        let updated_score = (600.0 * variance) as i64;
        let games = earned_xp.expected_games(cstreak::Game::Deathmatch {
            score: updated_score,
        }, 0);

        let duration = std::time::Duration::from_secs(games as u64 * 10 * 60);

        println!(
            "Needed ({:.0}% - score {})",
            (variance - 1.0) * 100.0,
            updated_score
        );
        println!("Games: {}", games);

        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        println!(
            "Duration: {}:{} (total: {}s)",
            hours,
            minutes,
            duration.as_secs()
        );
    }
}
