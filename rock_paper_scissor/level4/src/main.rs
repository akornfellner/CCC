fn main() {
    for i in 0..=5 {
        solve(i);
    }
}

fn solve(number: i32) {
    let filename: String = format!("input/level4_{}.in", number);
    let input = std::fs::read_to_string(filename).unwrap();

    let mut output = String::new();

    for line in input.lines().skip(1) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let values = [
            get_value(parts[0]),
            get_value(parts[1]),
            get_value(parts[2]),
        ];

        let t = set_tournament(&values);
        output.push_str(&t);
        output.push('\n');
        if tournament(&t) != 'S' {
            panic!("Invalid tournament");
        }
    }

    let output = output.trim();
    let output_filename: String = format!("output/level4_{}.out", number);

    std::fs::write(output_filename, output).unwrap();
}

fn get_value(input: &str) -> i32 {
    input
        .chars()
        .take(input.len() - 1)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn set_tournament(values: &[i32]) -> String {
    let mut r = values[0];
    let mut p = values[1];
    let mut s = values[2];

    let mut n = r + p + s;

    if n == 1 {
        return "S".to_string();
    }

    n /= 2;

    let mut result = String::new();

    if p > 0 {
        result.push('P');
        p -= 1;
        n -= 1;
    }

    while n > 0 && r > 0 {
        result.push('R');
        r -= 1;
        n -= 1;
    }

    while n > 0 && s > 1 {
        result.push('S');
        s -= 1;
        n -= 1;
    }

    while n > 0 && p > 0 {
        result.push('P');
        p -= 1;
        n -= 1;
    }

    result + &set_tournament(&[r, p, s])
}

fn winner(a: char, b: char) -> char {
    match (a, b) {
        ('R', 'S') | ('S', 'R') | ('R', 'R') => 'R',
        ('P', 'S') | ('S', 'P') | ('S', 'S') => 'S',
        ('P', 'R') | ('R', 'P') | ('P', 'P') => 'P',
        _ => a,
    }
}

fn tournament(input: &str) -> char {
    let mut fighters = input.to_string();

    while fighters.len() > 1 {
        let mut new_fighters = String::new();

        for i in (0..fighters.len() - 1).step_by(2) {
            let a = fighters.chars().nth(i).unwrap();
            let b = fighters.chars().nth(i + 1).unwrap();
            let winner = winner(a, b);
            new_fighters.push(winner);
        }

        fighters = new_fighters;
    }

    fighters.chars().next().unwrap()
}
