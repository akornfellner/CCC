use field::Field;
use std::fs;

mod field;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        println!("{}", solve(line));
    }
}

fn solve(input: &str) -> i32 {
    let parts = input.split(' ').collect::<Vec<&str>>();

    let mut current = Field::new(parts[1]);
    let end = Field::new(parts[2]);

    let forbidden = parts[3].parse::<usize>().unwrap();

    let directions: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    let mut count = 0;

    while current != end {
        let mut direction = 0usize;
        let mut better = i32::MAX;

        for dir in directions {
            if dir != forbidden {
                let next = current.go(dir);
                if next.diff(&end) < better {
                    direction = dir;
                    better = next.diff(&end);
                }
            }
        }

        current = current.go(direction);
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("D6 D5 B2 4"), 4);
    }
}
