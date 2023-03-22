use car::Car;
use std::fs;

mod car;

fn main() {
    for i in 1..=3 {
        println!("{}", solve(&format!("input/level3-{}.in", i)));
    }
}

fn solve(filename: &str) -> String {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let parts = lines[0].split(',').collect::<Vec<&str>>();
    let north = parts[0].parse::<f64>().unwrap();
    let east = parts[1].parse::<f64>().unwrap();
    let south = parts[2].parse::<f64>().unwrap();
    let west = parts[3].parse::<f64>().unwrap();

    let mut cars = vec![];

    for line in lines.iter().skip(2) {
        let car = Car::new(line);
        if car.is_in(north, east, south, west) {
            cars.push(car);
        }
    }

    let mut output = String::new();

    for car in &cars {
        output.push_str(&format!("{},", car));
    }

    cars.sort();

    output = output.trim_matches(',').to_string();

    output
}
