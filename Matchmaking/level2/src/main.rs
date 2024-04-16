use std::{collections::HashMap, fs};

fn main() {
    for i in 0..=5 {
        let input = fs::read_to_string(format!("input/level2_{}.in", i)).unwrap();
        fs::write(format!("output/level2_{}.out", i), solve(&input)).unwrap();
    }
}

fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let first = lines[0].split_whitespace().collect::<Vec<&str>>();
    let (_games_count, players_count) = (
        first[0].parse::<usize>().unwrap(),
        first[1].parse::<usize>().unwrap(),
    );

    let mut players: HashMap<usize, usize> = HashMap::new();

    for i in 0..players_count {
        players.insert(i, 0);
    }

    for line in lines.iter().skip(1) {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let (p1, s1) = (parts[0], parts[1]);
        let (p2, s2) = (parts[2], parts[3]);

        if s1 > s2 {
            *players.entry(p1).or_insert(0) += 1;
        } else {
            *players.entry(p2).or_insert(0) += 1;
        }
    }

    let mut sorted_players: Vec<_> = players.iter().collect();
    sorted_players.sort_by(|a, b| {
        let cmp = b.1.cmp(a.1);
        if cmp == std::cmp::Ordering::Equal {
            a.0.cmp(b.0)
        } else {
            cmp
        }
    });

    let mut result = String::new();

    for (player, wins) in sorted_players {
        result.push_str(&format!("{} {}\n", player, wins));
    }

    result.trim().to_string()
}
