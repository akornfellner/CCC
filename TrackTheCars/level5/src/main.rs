use car::*;
use geo::{Contains, LineString, Point, Polygon};
use std::{collections::HashMap, fs};

mod car;

fn main() {
    for i in 1..=4 {
        println!("{}", solve(&format!("input/level5-{}.in", i)));
    }
}

fn solve(filename: &str) -> String {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].parse().unwrap();
    let m: usize = lines[n + 1].parse().unwrap();

    let mut points: Vec<Point<f64>> = vec![];

    for line in lines.iter().skip(1).take(n) {
        let parts: Vec<f64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        points.push(Point::new(parts[0], parts[1]));
    }

    let first = Polygon::new(LineString::from(points.clone()), vec![]);

    points.clear();

    for line in lines.iter().skip(n + 2).take(m) {
        let parts: Vec<f64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        points.push(Point::new(parts[0], parts[1]));
    }

    let second = Polygon::new(LineString::from(points.clone()), vec![]);

    let mut cars_in_first: HashMap<String, Time> = HashMap::new();
    let mut cars_in_second: HashMap<String, Time> = HashMap::new();

    let start = Time::new("09:30:00");
    let end = Time::new("10:45:00");

    for line in lines.iter().skip(n + m + 3) {
        let car = Car::new(line);
        if start <= car.time && car.time <= end {
            if first.contains(&car.location) {
                let time: &Time = cars_in_first.entry(car.id.clone()).or_insert(car.time);
                if car.time < *time {
                    cars_in_first.insert(car.id, car.time);
                }
            } else if second.contains(&car.location) {
                let time: &Time = cars_in_second.entry(car.id.clone()).or_insert(car.time);
                if car.time > *time {
                    cars_in_second.insert(car.id, car.time);
                }
            }
        }
    }

    let mut cars: Vec<String> = vec![];

    for (car, time) in &cars_in_first {
        if cars_in_second.contains_key(car) && cars_in_second[car] > *time {
            cars.push(car.clone());
        }
    }

    cars.sort();

    let mut output = String::new();

    for car in &cars {
        output.push_str(&format!("{},", car));
    }

    output = output.trim_matches(',').to_string();

    output
}
