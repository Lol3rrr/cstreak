use crate::EarnedXp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Profile {
    pub level: i64,
    pub xp: i64,
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
}
