fn main() {
    for i in 0..6 {
        solve(i);
    }
}

fn solve(number: i32) {
    let filename: String = format!("input/level3_{}.in", number);
    let input = std::fs::read_to_string(filename).unwrap();

    let mut output = String::new();

    for line in input.lines().skip(1) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let values = [
            get_value(parts[0]),
            get_value(parts[1]),
            get_value(parts[2]),
        ];

        output.push_str(&get_pairs(&values));
        output.push('\n');
    }

    let output = output.trim();
    let output_filename: String = format!("output/level3_{}.out", number);

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

fn get_pairs(values: &[i32]) -> String {
    let mut output = String::new();
    let (mut r, mut p, mut s) = (values[0], values[1], values[2]);

    while r >= 3 && p > 0 {
        output.push_str("RRRP");
        r -= 3;
        p -= 1;
    }

    while r > 0 && p > 0 {
        output.push_str("RP");
        r -= 1;
        p -= 1;
    }

    while r > 0 && s > 0 {
        output.push_str("RS");
        r -= 1;
        s -= 1;
    }

    while p > 1 {
        output.push_str("PP");
        p -= 2;
    }

    while p > 0 && s > 0 {
        output.push_str("PS");
        p -= 1;
        s -= 1;
    }

    while s > 1 {
        output.push_str("SS");
        s -= 2;
    }

    output
}
