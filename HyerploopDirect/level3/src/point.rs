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

    pub fn new_coord(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_angle(&self) -> f64 {
        let angle = (self.y as f64 / self.x as f64).atan();

        if angle < 0.0 {
            angle + std::f64::consts::PI
        } else {
            angle
        }
    }

    pub fn is_same_side(&self, y: i32) -> bool {
        if y < 0 {
            self.y > y
        } else {
            self.y < y
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
