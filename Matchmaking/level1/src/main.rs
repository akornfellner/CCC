use std::fs;

fn main() {
    for i in 0..=5 {
        let input = fs::read_to_string(format!("input/level1_{}.in", i)).unwrap();
        fs::write(format!("output/level1_{}.out", i), solve(&input)).unwrap();
    }
}

fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let first = lines[0].split_whitespace().collect::<Vec<&str>>();
    let (_games_count, players_count) = (
        first[0].parse::<usize>().unwrap(),
        first[1].parse::<usize>().unwrap(),
    );

    let mut players = vec![0; players_count];

    for line in lines.iter().skip(1) {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if parts[1] > players[parts[0]] {
            players[parts[0]] = parts[1];
        }

        if parts[3] > players[parts[2]] {
            players[parts[2]] = parts[3];
        }
    }

    let mut max_score = 0;
    let mut max_index = 0;

    for (i, value) in players.iter().enumerate() {
        if value > &max_score || value == &max_score && i < max_index {
            max_score = *value;
            max_index = i;
        }
    }

    format!("{} {}", max_index, max_score)
}
