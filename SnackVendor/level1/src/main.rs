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
        format!("CHANGE {}", sum - amount)
    } else {
        format!("MISSING {}", amount - sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(solve("99 4 50 20 20 20"), "CHANGE 11");
    }

    #[test]
    fn test_missing() {
        assert_eq!(solve("99 4 50 5 10 10"), "MISSING 24");
    }
}
