use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\n\n").collect();

    let mut honeycombs: Vec<Honeycomb> = vec![];

    for h in input.iter().skip(1) {
        let mut honeycomb: Honeycomb = vec![];
        for line in h.lines() {
            let mut row: Vec<Cell> = vec![];
            for c in line.chars() {
                match c {
                    '-' => row.push(Cell::Space),
                    'X' => row.push(Cell::Barrier),
                    'O' => row.push(Cell::Empty),
                    _ => row.push(Cell::Wasp),
                }
            }
            honeycomb.push(row);
        }
        honeycombs.push(honeycomb);
    }

    let mut output: String = String::new();

    for honeycomb in &honeycombs {
        let (x, y) = get_wasp(honeycomb);

        let mut visited: Vec<(usize, usize)> = vec![];

        if is_free(x, y, honeycomb, &mut visited) {
            output.push_str("FREE\n");
        } else {
            output.push_str("TRAPPED\n");
        }
    }

    output = output.trim_matches('\n').to_string();
    fs::write("output", output).unwrap();
}

fn is_free(x: usize, y: usize, honeycomb: &Honeycomb, visited: &mut Vec<(usize, usize)>) -> bool {
    visited.push((x, y));
    if let Cell::Barrier = honeycomb[x][y] {
        return false;
    }

    if x == 0 || y <= 1 || x >= honeycomb.len() - 1 || y >= honeycomb[0].len() - 2 {
        return true;
    }

    let neighbors: Vec<(usize, usize)> = vec![
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x, y - 2),
        (x, y + 2),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ];

    let mut free = false;

    for (i, j) in neighbors {
        if visited.contains(&(i, j)) {
            continue;
        }
        free = free || is_free(i, j, honeycomb, visited);
    }

    free
}

fn get_wasp(honeycomb: &Honeycomb) -> (usize, usize) {
    for (i, row) in honeycomb.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Wasp = cell {
                return (i, j);
            }
        }
    }
    panic!("No wasp found")
}

type Honeycomb = Vec<Vec<Cell>>;

enum Cell {
    Empty,
    Space,
    Wasp,
    Barrier,
}
