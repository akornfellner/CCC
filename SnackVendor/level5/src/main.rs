use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        println!("{}", solve(line));
    }
}

fn solve(input: &str) -> i32 {
    let parts = input.split(' ').collect::<Vec<&str>>();

    let first = parse_field(parts[1]);
    let second = parse_field(parts[2]);

    let x = (first.0 as i32 - second.0 as i32).abs();
    let y = (first.1 as i32 - second.1 as i32).abs();

    x.max(y)
}

fn parse_field(input: &str) -> (usize, usize) {
    let input = input.to_string();
    let x = (input.chars().collect::<Vec<char>>()[0]) as usize - 'A' as usize;
    let y = input[1..].parse::<usize>().unwrap() - 1;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("D6 D5 B2"), 3);
    }
}
