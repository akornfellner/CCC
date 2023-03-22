use std::fmt::Display;

#[derive(Debug)]
pub struct Car {
    pub id: String,
    pub time: String,
    pub x: f64,
    pub y: f64,
}

impl Car {
    pub fn new(input: &str) -> Self {
        let parts = input.split(',').collect::<Vec<&str>>();
        Car {
            id: parts[0].to_string(),
            time: parts[1].to_string(),
            x: parts[2].parse::<f64>().unwrap(),
            y: parts[3].parse::<f64>().unwrap(),
        }
    }

    pub fn is_in(&self, north: f64, east: f64, south: f64, west: f64) -> bool {
        self.x < north && self.x > south && self.y < east && self.y > west
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Car {}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Car {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
