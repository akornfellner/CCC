use geo::Point;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn new(input: &str) -> Self {
        let parts: Vec<u8> = input.split(':').map(|x| x.parse().unwrap()).collect();
        Self {
            hour: parts[0],
            minute: parts[1],
            second: parts[2],
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else if self.minute != other.minute {
            self.minute.cmp(&other.minute)
        } else {
            self.second.cmp(&other.second)
        }
    }
}

#[derive(Debug)]
pub struct Car {
    pub id: String,
    pub time: Time,
    pub location: Point<f64>,
}

impl Car {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(',').collect();
        Self {
            id: parts[0].to_string(),
            time: Time::new(parts[1]),
            location: Point::new(parts[2].parse().unwrap(), parts[3].parse().unwrap()),
        }
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Car {}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Car {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.id.cmp(&other.id);

        if cmp == Ordering::Equal {
            self.time.cmp(&other.time)
        } else {
            cmp
        }
    }
}
