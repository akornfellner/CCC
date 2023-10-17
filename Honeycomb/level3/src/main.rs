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

    let neighbors: Vec<(i32, i32)> = vec![(-1, -1), (-1, 1), (0, -2), (0, 2), (1, -1), (1, 1)];

    let mut output: String = String::new();

    for honeycomb in &honeycombs {
        let (x, y) = get_wasp(honeycomb);
        let mut is_free = false;
        'outer: for neighbor in &neighbors {
            let (mut x, mut y) = (x, y);
            'inner: loop {
                let next_x = x as i32 + neighbor.0;
                let next_y = y as i32 + neighbor.1;

                if next_x < 0
                    || next_y < 0
                    || next_x >= honeycomb.len() as i32
                    || next_y >= honeycomb[0].len() as i32
                {
                    match honeycomb[x][y] {
                        Cell::Empty => {
                            is_free = true;
                            break 'outer;
                        }
                        _ => break 'inner,
                    }
                }

                (x, y) = (next_x as usize, next_y as usize);

                match honeycomb[x][y] {
                    Cell::Empty => {
                        continue 'inner;
                    }
                    _ => break 'inner,
                }
            }
        }

        if is_free {
            output.push_str("FREE\n");
        } else {
            output.push_str("TRAPPED\n");
        }
    }

    output = output.trim_matches('\n').to_string();
    fs::write("output", output).unwrap();
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
