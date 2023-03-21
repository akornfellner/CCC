use crate::obstacle::Obstacle;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            visible: true,
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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let angle = self.get_angle();
        let other_angle = other.get_angle();

        if angle < other_angle {
            std::cmp::Ordering::Less
        } else if angle > other_angle {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
