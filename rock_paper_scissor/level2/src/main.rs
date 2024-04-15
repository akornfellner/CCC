fn main() {
    for i in 0..6 {
        solve(i);
    }
}

fn solve(number: i32) {
    let filename: String = format!("input/level2_{}.in", number);
    let input = std::fs::read_to_string(filename).unwrap();

    let mut output = String::new();

    for line in input.lines().skip(1) {
        let mut duels = line.to_string();
        for _ in 0..2 {
            duels = round(&duels);
        }
        output.push_str(&duels);
        output.push('\n');
    }

    let output = output.trim();
    let output_filename: String = format!("output/level2_{}.out", number);

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

fn round(duels: &str) -> String {
    let mut new_duels = String::new();

    for i in (0..duels.len() - 1).step_by(2) {
        let duel = duel(&duels[i..i + 2]);
        new_duels.push(duel);
    }

    new_duels
}
