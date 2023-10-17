use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\n\n").collect();

    let mut honeycombs: Vec<Honeycomb> = vec![];

    for h in input.iter().skip(1) {
        honeycombs.push(Honeycomb::new(h));
    }

    for honeycomb in &honeycombs {
        println!("{}", honeycomb.to_string());
    }
}

fn is_free(x: usize, y: usize, honeycomb: &Honeycomb, visited: &mut Vec<(usize, usize)>) -> bool {
    visited.push((x, y));
    if let Cell::Barrier = honeycomb.field[x][y] {
        return false;
    }

    if x == 0 || y <= 1 || x >= honeycomb.field.len() - 1 || y >= honeycomb.field[0].len() - 2 {
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
    for (i, row) in honeycomb.field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Wasp = cell {
                return (i, j);
            }
        }
    }
    panic!("No wasp found")
}

#[derive(Debug)]
struct Honeycomb {
    field: Vec<Vec<Cell>>,
    n: usize,
}

impl Honeycomb {
    fn new(input: &str) -> Self {
        let mut field: Vec<Vec<Cell>> = vec![];

        let n = input.lines().nth(0).unwrap().parse::<usize>().unwrap();

        for line in input.lines().skip(1) {
            let mut row: Vec<Cell> = vec![];
            for c in line.chars() {
                match c {
                    '-' => row.push(Cell::Space),
                    'X' => row.push(Cell::Barrier),
                    'O' => row.push(Cell::Empty),
                    _ => row.push(Cell::Wasp),
                }
            }
            field.push(row);
        }

        Self { field, n }
    }

    fn to_string(&self) -> String {
        let mut result = String::from(self.n.to_string());
        for row in self.field.iter() {
            result += "\n";
            for cell in row.iter() {
                match cell {
                    Cell::Empty => result += "O",
                    Cell::Space => result += "-",
                    Cell::Wasp => result += "W",
                    Cell::Barrier => result += "X",
                }
            }
        }
        result
    }
}

#[derive(Debug)]
enum Cell {
    Empty,
    Space,
    Wasp,
    Barrier,
}
