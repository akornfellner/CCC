#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub x: i32,
    pub y: i32,
}

impl Field {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        let x = (input.chars().collect::<Vec<char>>()[0]) as i32 - 'A' as i32;
        let y = input[1..].parse::<i32>().unwrap() - 1;

        Self { x, y }
    }

    pub fn go(&self, direction: usize) -> Self {
        match direction {
            0 => Self {
                x: self.x,
                y: self.y + 1,
            },
            1 => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            2 => Self {
                x: self.x - 1,
                y: self.y,
            },
            3 => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            4 => Self {
                x: self.x,
                y: self.y - 1,
            },
            5 => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            6 => Self {
                x: self.x + 1,
                y: self.y,
            },
            _ => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }

    pub fn diff(&self, other: &Self) -> i32 {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();

        x + y
    }
}
