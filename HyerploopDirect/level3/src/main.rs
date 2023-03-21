use point::Point;
use std::fs;

mod point;

fn main() {
    let input = String::from("input/level3");

    for i in 1..=4 {
        let input = format!("{input}-{i}.txt");
        println!("{}", solve(&input));
    }
}

fn solve(input: &str) -> String {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let parts = lines[0]
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let y = parts[2];
    let mut a1 = Point::new_coord(parts[0], y).get_angle();
    let mut a2 = Point::new_coord(parts[1], y).get_angle();

    if a1 > a2 {
        (a1, a2) = (a2, a1);
    }

    let mut output = String::new();

    for line in lines.iter().skip(2) {
        let point = Point::new(line);
        let angle = point.get_angle();

        if angle < a1 || angle > a2 || point.is_same_side(y) {
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
        assert_eq!(
            solve("input/level3-eg.txt"),
            "2 -5 2 4 -1 2 -2 2 -5 4".to_string()
        );
    }
}
