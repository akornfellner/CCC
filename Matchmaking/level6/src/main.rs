use std::{collections::HashMap, fs};

fn main() {
    for i in 0..=5 {
        let input = fs::read_to_string(format!("input/level6_{}.in", i)).unwrap();
        fs::write(format!("output/level6_{}.out", i), solve(&input)).unwrap();
    }
}

fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let first = lines[0].split_whitespace().collect::<Vec<&str>>();
    let (games_count, players_count, size) = (
        first[0].parse::<usize>().unwrap(),
        first[1].parse::<usize>().unwrap(),
        first[2].parse::<usize>().unwrap(),
    );

    let mut players: HashMap<usize, i32> = HashMap::new();

    for i in 0..players_count {
        players.insert(i, 1000);
    }

    for line in lines.iter().skip(1).take(games_count) {
        let (team1, team2, win) = parse_line(line, size);
        let (ratings1, ratings2) = get_ratings(&team1, &team2, &players, win);

        for (i, player) in team1.iter().enumerate() {
            players.insert(*player, ratings1[i]);
        }

        for (i, player) in team2.iter().enumerate() {
            players.insert(*player, ratings2[i]);
        }
    }

    let a = lines
        .iter()
        .skip(games_count + 1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{:?}", a);

    let (threshold, max_score) = (0, 0);

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

fn parse_line(line: &str, size: usize) -> (Vec<usize>, Vec<usize>, bool) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let mut team1: Vec<usize> = vec![];
    let mut team2: Vec<usize> = vec![];

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in (0..parts.len() - 1).step_by(2) {
        if i / 2 < size {
            team1.push(parts[i].parse().unwrap());
            sum1 += parts[i + 1].parse::<usize>().unwrap();
        } else {
            team2.push(parts[i].parse().unwrap());
            sum2 += parts[i + 1].parse::<usize>().unwrap();
        }
    }

    (team1, team2, sum1 > sum2)
}

fn get_ratings(
    team1: &[usize],
    team2: &[usize],
    players: &HashMap<usize, i32>,
    win: bool,
) -> (Vec<i32>, Vec<i32>) {
    let mut ratings1 = vec![];
    let mut ratings2 = vec![];

    let s1 = team1.iter().map(|&x| players[&x]).sum::<i32>();
    let s2 = team2.iter().map(|&x| players[&x]).sum::<i32>();

    for player in team1 {
        let o1 = team1
            .iter()
            .filter(|&&x| x != *player)
            .map(|&x| players[&x])
            .sum::<i32>();
        let e = winning_chance(players[player], s2 - o1);
        ratings1.push(update_score(players[player], 32.0, e, win));
    }

    for player in team2 {
        let o2 = team2
            .iter()
            .filter(|&&x| x != *player)
            .map(|&x| players[&x])
            .sum::<i32>();
        let e = winning_chance(players[player], s1 - o2);
        ratings2.push(update_score(players[player], 32.0, e, !win));
    }

    (ratings1, ratings2)
}

fn winning_chance(s1: i32, s2: i32) -> f64 {
    1.0 / (1.0 + 10f64.powf((s2 as f64 - s1 as f64) / 400.0))
}

fn update_score(s1: i32, k: f64, e1: f64, win: bool) -> i32 {
    let win = if win { 1.0 } else { 0.0 };

    (s1 as f64 + k * (win - e1)).floor() as i32
}
