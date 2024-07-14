mod account;
pub use account::Profile;

const XP_STEPS: [(i64, i64); 4] = [(4500, 4), (7500, 2), (11167, 1), (i64::MAX, 0)];

pub fn total_target() -> i64 {
    XP_STEPS[2].0
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Game {
    Deathmatch { score: i64 },
}

impl Game {
    pub fn xp(&self) -> i64 {
        match self {
            Self::Deathmatch { score } => score / 5,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct EarnedXp(pub i64);

impl EarnedXp {
    pub fn expected_games(&self, expected_game: Game) -> i64 {
        let mut games = 0;
        let mut current_earned = self.0;

        for window in XP_STEPS.windows(2) {
            assert_eq!(2, window.len());
            let (first_thres, first_multiplier) = window[0];
            let (_, second_multiplier) = window[1];

            if current_earned < first_thres {
                let until_threshold = first_thres - current_earned;
                let multiplied_games = until_threshold / (expected_game.xp() * first_multiplier);

                games += multiplied_games;
                current_earned += multiplied_games * expected_game.xp() * first_multiplier;

                let remaining_until_threshold = first_thres - current_earned;
                if remaining_until_threshold > 0 {
                    let earn_in_first = remaining_until_threshold / first_multiplier;
                    let earn_in_second = expected_game.xp() - earn_in_first;

                    games += 1;
                    current_earned +=
                        earn_in_first * first_multiplier + earn_in_second * second_multiplier;
                }
            }
        }

        games
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: i64 = XP_STEPS[2].0;
    const THRESHOLD_2TIMES: i64 = XP_STEPS[1].0;
    const THRESHOLD_4TIMES: i64 = XP_STEPS[0].0;

    #[test]
    fn games_after_bonuses() {
        assert_eq!(
            2,
            EarnedXp(TARGET - 200).expected_games(Game::Deathmatch { score: 500 })
        );
    }
    #[test]
    fn games_after_bonuses_not_exact() {
        assert_eq!(
            3,
            EarnedXp(TARGET - 250).expected_games(Game::Deathmatch { score: 500 })
        );
    }

    #[test]
    fn games_with_2x() {
        assert_eq!(
            38,
            EarnedXp(THRESHOLD_2TIMES - 233).expected_games(Game::Deathmatch { score: 500 })
        );
    }
    #[test]
    fn games_with_2x_exact_until_threshold() {
        assert_eq!(
            38,
            EarnedXp(THRESHOLD_2TIMES - 200).expected_games(Game::Deathmatch { score: 500 })
        );
    }
    #[test]
    fn games_with_2x_not_exact() {
        assert_eq!(
            38,
            EarnedXp(THRESHOLD_2TIMES - 250).expected_games(Game::Deathmatch { score: 500 })
        );
    }

    #[test]
    fn games_with_4x() {
        assert_eq!(
            53,
            EarnedXp(THRESHOLD_4TIMES - 233).expected_games(Game::Deathmatch { score: 500 })
        );
    }
    #[test]
    fn games_with_4x_exact_until_threshold() {
        assert_eq!(
            53,
            EarnedXp(THRESHOLD_4TIMES - 200).expected_games(Game::Deathmatch { score: 500 })
        );
    }
    #[test]
    fn games_with_4x_not_exact() {
        assert_eq!(
            53,
            EarnedXp(THRESHOLD_4TIMES - 250).expected_games(Game::Deathmatch { score: 500 })
        );
    }

    #[test]
    fn profile_dif() {
        assert_eq!(
            EarnedXp(6000),
            Profile { level: 5, xp: 1000 }.earned_xp(&Profile { level: 6, xp: 2000 })
        );
    }
    #[test]
    fn profile_dif_wrapping() {
        assert_eq!(
            EarnedXp(11000),
            Profile {
                level: 38,
                xp: 1000
            }
            .earned_xp(&Profile { level: 1, xp: 2000 })
        );
    }
}
