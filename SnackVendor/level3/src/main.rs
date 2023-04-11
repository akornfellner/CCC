use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        println!("{}", solve(line));
    }
}

fn solve(input: &str) -> String {
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    let size = parts[0].to_string();

    let rows: usize = (size.chars().collect::<Vec<char>>()[0]) as usize - 'A' as usize + 1;
    let cols: usize = size[1..].parse::<usize>().unwrap();

    let mut values = parts.iter().skip(1).take(rows * cols);

    let mut matrix = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = values.next().unwrap().parse::<i32>().unwrap();
        }
    }

    let field = parts[rows * cols + 1].to_string();

    let row: usize = (field.chars().collect::<Vec<char>>()[0]) as usize - 'A' as usize;
    let col: usize = field[1..].parse::<usize>().unwrap() - 1;

    let sum: i32 = parts[rows * cols + 3..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    let amount = matrix[row][col];

    if sum > amount {
        format!("CHANGE {}", coins_return(sum - amount))
    } else {
        format!("MISSING {}", amount - sum)
    }
}

fn coins_return(amount: i32) -> String {
    let mut amount = amount;
    let coins = [200, 100, 50, 20, 10, 5, 2, 1];

    let mut values: Vec<i32> = vec![];

    for coin in &coins {
        let count = amount / coin;
        if count > 0 {
            amount -= count * coin;
        }

        values.insert(0, count);
    }

    let mut result = String::new();

    values
        .iter()
        .for_each(|x| result.push_str(&format!("{} ", x)));

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(
            solve("C2 79 65 99 129 149 199 B1 5 50 20 20 20 20"),
            "CHANGE 1 0 0 1 1 0 0 0"
        );
    }
}
