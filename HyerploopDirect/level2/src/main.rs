use point::Point;
use std::fs;

mod point;

fn main() {
    let input = String::from("input/level2");

    for i in 1..=4 {
        let input = format!("{input}-{i}.txt");
        println!("{}", solve(&input));
    }
}

fn solve(input: &str) -> String {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let first = Point::new(lines[0]);
    let second = Point::new(lines[1]);

    let mut a1 = first.get_angle();
    let mut a2 = second.get_angle();

    if a1 > a2 {
        (a1, a2) = (a2, a1);
    }

    let mut output = String::new();

    for line in lines.iter().skip(3) {
        let angle = Point::new(line).get_angle();

        if angle < a1 || angle > a2 {
            output.push_str(line);
            output.push(' ');
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
        assert_eq!(solve("input/level2-eg.txt"), "4 3 -5 1".to_string());
    }
}
