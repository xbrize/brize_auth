use std::time::{SystemTime, UNIX_EPOCH};

/// A utility for creating expiration times in UNIX_EPOCH format
pub enum Expiry {
    Second(u64),
    Day(u64),
    Week(u64),
    Month(u64),
    Year(u64),
}

impl Expiry {
    /// Unwraps the count and converts to the UNIX_EPOCH expiration time
    pub fn time(&self) -> u64 {
        match self {
            Expiry::Second(count) => Self::set_expiration(*count),
            Expiry::Day(count) => Self::day_expiry(*count),
            Expiry::Week(count) => Self::week_expiry(*count),
            Expiry::Month(count) => Self::month_expiry(*count),
            Expiry::Year(count) => Self::year_expiry(*count),
        }
    }

    /// Current time in UNIX_EPOCH
    pub fn now() -> u64 {
        Self::current_time_in_epoch()
    }

    /// Checks if this Expiry has become expired
    pub fn is_expired(&self) -> bool {
        self.time() < Self::current_time_in_epoch()
    }

    fn set_expiration(seconds: u64) -> u64 {
        Self::current_time_in_epoch() + seconds
    }

    fn current_time_in_epoch() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        since_the_epoch.as_secs()
    }

    fn day_expiry(count: u64) -> u64 {
        let day_in_sec = 86_400;
        Self::set_expiration(day_in_sec * count)
    }

    fn week_expiry(count: u64) -> u64 {
        let week_in_sec = 604_800;
        Self::set_expiration(week_in_sec * count)
    }

    fn month_expiry(count: u64) -> u64 {
        let month_in_sec = 2_678_400;
        Self::set_expiration(month_in_sec * count)
    }

    fn year_expiry(count: u64) -> u64 {
        let year_in_sec = 31_536_000;
        Self::set_expiration(year_in_sec * count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expiry_entity() {
        let exp = Expiry::Day(1);
        assert!(!exp.is_expired());
    }
}
