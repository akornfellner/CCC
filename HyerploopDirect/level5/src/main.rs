use obstacle::Obstacle;
use point::Point;
use std::fs;

mod obstacle;
mod point;

fn main() {
    let input = String::from("input/level5");

    for i in 1..=4 {
        let input = format!("{input}-{i}.txt");
        println!("{}", solve(&input));
    }
}

fn solve(input: &str) -> i32 {
    let input = fs::read_to_string(input).expect("Something went wrong reading the file");
    let lines = input.lines().collect::<Vec<&str>>();

    let s: i32 = lines[0].parse().unwrap();

    let mut obstacles: Vec<Obstacle> = vec![];

    for line in lines.iter().skip(2) {
        obstacles.push(Obstacle::new(line));
    }

    obstacles.sort();

    let mut points: Vec<Point> = vec![];

    for x in -s..=s {
        for y in -s..=s {
            points.push(Point::new(x, y));
        }
    }

    points.sort();

    let mut count = 0;

    let mut skip = 0; // is used to skip points that are already behind an obstacle

    for obstacle in &obstacles {
        let (start, end) = obstacle.get_angles();

        for point in points.iter_mut().skip(skip) {
            let angle = point.get_angle();

            if angle < start {
                skip += 1;
                continue;
            }
            if angle > end {
                break;
            }

            if point.is_behind_obstacle(obstacle) {
                point.visible = false;
            }
        }
    }

    for point in &points {
        if point.visible {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("input/level5-eg.txt"), 62);
    }
}
