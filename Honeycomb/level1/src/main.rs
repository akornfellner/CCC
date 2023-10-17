use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cells: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        cells.push(row);
    }

    let mut count = 0;

    for line in &cells {
        for c in line {
            if *c == 'O' {
                count += 1;
            }
        }
    }

    match fs::write("output", count.to_string()) {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Error!"),
    }
}
