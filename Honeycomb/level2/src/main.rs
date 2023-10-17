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
        let mut count = 0;
        let (x, y) = get_wasp(honeycomb);
        for neighbour in get_neighbours(x, y) {
            if let Cell::Empty = honeycomb[neighbour.0][neighbour.1] {
                count += 1
            }
        }
        output.push_str(&format!("{}\n", count));
    }

    output = output.trim_matches('\n').to_string();
    fs::write("output", output).unwrap();
}

fn get_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let neighbours: Vec<(usize, usize)> = vec![
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x, y - 2),
        (x, y + 2),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ];
    neighbours
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
