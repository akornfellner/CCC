use crate::point::Point;

#[derive(Debug)]
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
        let start = Point {
            x: self.start_x,
            y: self.y,
        };
        let end = Point {
            x: self.end_x,
            y: self.y,
        };
        if start.get_angle() > end.get_angle() {
            (end.get_angle(), start.get_angle())
        } else {
            (start.get_angle(), end.get_angle())
        }
    }
}
