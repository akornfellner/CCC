use crate::obstacle::Obstacle;
use std::fmt::Display;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(input: &str) -> Point {
        let parts = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        Point {
            x: parts[0],
            y: parts[1],
        }
    }

    pub fn get_angle(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }

    fn is_same_side(&self, y: i32) -> bool {
        if y < 0 {
            self.y > y
        } else {
            self.y < y
        }
    }

    pub fn is_behind_obstacle(&self, obstacle: &Obstacle) -> bool {
        let (start, end) = obstacle.get_angles();
        let angle = self.get_angle();

        !(angle < start || angle > end || self.is_same_side(obstacle.y))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
