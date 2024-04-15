fn main() {
    for i in 0..6 {
        solve(i);
    }
}

fn solve(number: i32) {
    let filename: String = format!("input/level1_{}.in", number);
    let input = std::fs::read_to_string(filename).unwrap();

    let _n = input.lines().next().unwrap().parse::<i32>().unwrap();
    let mut output = String::new();

    for line in input.lines().skip(1) {
        let duel = duel(line);
        output.push(duel);
        output.push('\n');
    }

    let output = output.trim();
    let output_filename: String = format!("output/level1_{}.out", number);

    std::fs::write(output_filename, output).unwrap();
}

fn duel(duel: &str) -> char {
    let mut chars: Vec<char> = duel.chars().collect();
    chars.sort();

    let first = chars[0];
    let second = chars[1];

    if first == 'P' && second == 'S' {
        'S'
    } else {
        first
    }
}
