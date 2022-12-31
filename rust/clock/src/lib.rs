use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
