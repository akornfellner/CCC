use geo::{Contains, LineString, Point, Polygon};
use std::fs;

fn main() {
    for i in 1..=4 {
        println!("{}", solve(&format!("input/level4-{}.in", i)));
    }
}

fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].parse().unwrap();

    let mut points: Vec<Point<f64>> = vec![];

    for line in lines.iter().skip(1).take(n) {
        let parts: Vec<f64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        points.push(Point::new(parts[0], parts[1]));
    }

    let polygon: Polygon<f64> = Polygon::new(LineString::from(points), vec![]);

    let mut count: usize = 0;

    for line in lines.iter().skip(n + 2) {
        let parts: Vec<f64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let point = Point::new(parts[0], parts[1]);

        if polygon.contains(&point) {
            count += 1;
        }
    }

    count
}
