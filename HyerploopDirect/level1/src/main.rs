use point::Point;
use std::fs;

mod point;

fn main() {
    let input = String::from("input/level1");

    for i in 1..=4 {
        let input = format!("{input}-{i}.txt");
        println!("{}", solve(&input));
    }
}

fn solve(input: &str) -> String {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let y: i32 = lines[0].parse().unwrap();

    let mut output = String::new();

    for line in lines.iter().skip(2) {
        let point = Point::new(line);

        if y > 0 && y > point.y || y < 0 && y < point.y {
            output.push_str(&format!("{} ", point));
        }
    }

    output = output.trim().to_string();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("input/level1-eg.txt"), "2 -5 -1 2 -2 2".to_string());
    }
}
