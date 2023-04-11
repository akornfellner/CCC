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
    let (rows, cols) = parse_field(parts[0]);
    let rows = rows + 1;
    let cols = cols + 1;

    let mut prices = vec![vec![0; cols]; rows];
    let mut values = vec![vec![0; cols]; rows];

    let mut iter = parts[1..].iter().map(|x| x.parse::<i32>().unwrap());

    for row in prices.iter_mut() {
        for col in row.iter_mut() {
            *col = iter.next().unwrap();
        }
    }

    for row in values.iter_mut() {
        for col in row.iter_mut() {
            *col = iter.next().unwrap();
        }
    }

    let mut amount = 0;

    for field in parts.iter().skip(rows * cols * 2 + 2) {
        let (x, y) = parse_field(field);
        if values[x][y] > 0 {
            amount += prices[x][y];
            values[x][y] -= 1;
        }
    }

    amount
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
    fn test_change() {
        assert_eq!(
            solve("C2 79 65 99 129 149 199 3 2 2 1 3 4 9 B2 A1 B2 A2 B1 A2 B2 C2 A2"),
            636
        );
    }
}
