use std::{collections::HashMap, fs};

fn main() {
    for i in 0..=5 {
        let input = fs::read_to_string(format!("input/level4_{}.in", i)).unwrap();
        fs::write(format!("output/level4_{}.out", i), solve(&input)).unwrap();
    }
}

fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let first = lines[0].split_whitespace().collect::<Vec<&str>>();
    let (_games_count, players_count) = (
        first[0].parse::<usize>().unwrap(),
        first[1].parse::<usize>().unwrap(),
    );

    let mut players: HashMap<usize, i32> = HashMap::new();

    for i in 0..players_count {
        players.insert(i, 1000);
    }

    for line in lines.iter().skip(1) {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let (p1, s1) = (parts[0], parts[1]);
        let (p2, s2) = (parts[2], parts[3]);

        let (e1, e2) = winning_chance(players[&p1], players[&p2]);

        if s1 > s2 {
            players.insert(p1, update_score(players[&p1], 32.0, e1, true));
            players.insert(p2, update_score(players[&p2], 32.0, e2, false));
        } else {
            players.insert(p1, update_score(players[&p1], 32.0, e1, false));
            players.insert(p2, update_score(players[&p2], 32.0, e2, true));
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

fn winning_chance(s1: i32, s2: i32) -> (f64, f64) {
    let e1 = 1.0 / (1.0 + 10f64.powf((s2 as f64 - s1 as f64) / 400.0));
    let e2 = 1.0 / (1.0 + 10f64.powf((s1 as f64 - s2 as f64) / 400.0));

    (e1, e2)
}

fn update_score(s1: i32, k: f64, e1: f64, win: bool) -> i32 {
    let win = if win { 1.0 } else { 0.0 };

    (s1 as f64 + k * (win - e1)).floor() as i32
}
