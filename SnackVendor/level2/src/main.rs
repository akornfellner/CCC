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

    let amount = parts[0].parse::<i32>().unwrap();

    let sum: i32 = parts[2..].iter().map(|x| x.parse::<i32>().unwrap()).sum();

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
        assert_eq!(solve("99 5 50 20 20 20 20"), "CHANGE 1 0 0 1 1 0 0 0");
    }

    #[test]
    fn test_missing() {
        assert_eq!(solve("99 4 50 5 10 10"), "MISSING 24");
    }
}
