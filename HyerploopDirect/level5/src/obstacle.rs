use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Obstacle {
    pub start_x: i32,
    pub end_x: i32,
    pub y: i32,
}

impl Obstacle {
    pub fn new(input: &str) -> Obstacle {
        let parts = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        Obstacle {
            start_x: parts[0],
            end_x: parts[1],
            y: parts[2],
        }
    }

    pub fn get_angles(&self) -> (f64, f64) {
        let start = Point::new(self.start_x, self.y);
        let end = Point::new(self.end_x, self.y);
        if start.get_angle() > end.get_angle() {
            (end.get_angle(), start.get_angle())
        } else {
            (start.get_angle(), end.get_angle())
        }
    }
}

impl PartialOrd for Obstacle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Obstacle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (a1, a2) = self.get_angles();
        let (b1, b2) = other.get_angles();

        if a1 < b1 {
            std::cmp::Ordering::Less
        } else if a1 > b1 {
            std::cmp::Ordering::Greater
        } else {
            if a2 < b2 {
                std::cmp::Ordering::Less
            } else if a2 > b2 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
