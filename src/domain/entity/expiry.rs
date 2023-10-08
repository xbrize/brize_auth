use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub enum Expiry {
    Second(u64),
    Day(u64),
    Week(u64),
    Month(u64),
    Year(u64),
}

impl Expiry {
    pub fn time(&self) -> u64 {
        match self {
            Expiry::Second(count) => Self::expiry_util(*count),
            Expiry::Day(count) => Self::day_expiry(*count),
            Expiry::Week(count) => Self::week_expiry(*count),
            Expiry::Month(count) => Self::month_expiry(*count),
            Expiry::Year(count) => Self::year_expiry(*count),
        }
    }

    pub fn now() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        since_the_epoch.as_secs()
    }

    pub fn is_expired(&self) -> bool {
        self.time() < Self::now()
    }

    pub fn is_past_epoch_seconds(seconds: usize) -> bool {
        seconds < Self::now() as usize
    }

    fn expiry_failure_fallback() -> u64 {
        10000000000
    }

    fn expiry_util(seconds: u64) -> u64 {
        let timestamp = Self::now() + seconds;

        match timestamp.try_into() {
            Ok(timestamp) => timestamp,
            Err(e) => {
                println!("Logging timestamp conversion error:{e}");
                Self::expiry_failure_fallback()
            }
        }
    }

    fn day_expiry(count: u64) -> u64 {
        let day_in_sec = 86_400;
        Self::expiry_util(day_in_sec * count)
    }

    fn week_expiry(count: u64) -> u64 {
        let week_in_sec = 604_800;
        Self::expiry_util(week_in_sec * count)
    }

    fn month_expiry(count: u64) -> u64 {
        let month_in_sec = 2_678_400;
        Self::expiry_util(month_in_sec * count)
    }

    fn year_expiry(count: u64) -> u64 {
        let year_in_sec = 31_536_000;
        Self::expiry_util(year_in_sec * count)
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
