use crate::EarnedXp;

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub level: i64,
    pub xp: i64,
}

impl Default for Profile {
    fn default() -> Self {
        Self { level: 1, xp: 0 }
    }
}

impl Profile {
    pub fn earned_xp(&self, other: &Self) -> EarnedXp {
        let level_dif = if other.level < self.level {
            other.level + (40 - self.level - 1)
        } else {
            other.level - self.level
        };

        let xp_dif = other.xp - self.xp;

        EarnedXp(level_dif * 5000 + xp_dif)
    }

    pub fn target_profile(&self, mission_xp: i64) -> Profile {
        let resulting_xp = self.xp + crate::total_target() + mission_xp;

        let level_delta = resulting_xp / 5000;

        let result_xp = resulting_xp % 5000;
        let result_level = self.level + level_delta;

        Profile {
            xp: result_xp,
            level: result_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_normal() {
        let start = Profile { level: 5, xp: 0 };

        let result = start.target_profile();

        let expected = Profile { level: 7, xp: 1167 };

        assert_eq!(expected, result);
    }
}
