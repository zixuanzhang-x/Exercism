use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_minutes = minutes;
        let mut new_hours = hours;

        if minutes < 0 {
            let borrowed_hours = minutes.abs() / 60 + 1;

            new_minutes = minutes + borrowed_hours * 60;

            new_hours -= borrowed_hours;
        }

        if new_hours < 0 {
            new_hours = new_hours + 24 * (new_hours.abs() / 24 + 1);
        }

        new_hours += new_minutes / 60;

        new_minutes %= 60;
        new_hours %= 24;

        Self {
            hours: new_hours,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
