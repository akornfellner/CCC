use obstacle::Obstacle;
use point::Point;
use std::fs;

mod obstacle;
mod point;

fn main() {
    let input = String::from("input/level4");

    for i in 1..=4 {
        let input = format!("{input}-{i}.txt");
        println!("{}", solve(&input));
    }
}

fn solve(input: &str) -> String {
    let input = fs::read_to_string(input).expect("Something went wrong reading the file");
    let lines = input.lines().collect::<Vec<&str>>();

    let v: i32 = lines[0].parse().unwrap();

    let mut obstacles: Vec<Obstacle> = vec![];

    for line in lines.iter().skip(1).take(v as usize) {
        obstacles.push(Obstacle::new(line));
    }

    let mut points: Vec<Point> = vec![];

    for line in lines.iter().skip(2 + v as usize) {
        points.push(Point::new(line));
    }

    let mut output = String::new();

    for point in &points {
        let mut behind = false;
        for obstacle in &obstacles {
            if point.is_behind_obstacle(obstacle) {
                behind = true;
                break;
            }
        }
        if !behind {
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
        assert_eq!(solve("input/level4-eg.txt"), "1 2 -4 0 -1 -3".to_string());
    }
}
