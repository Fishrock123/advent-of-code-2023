use std::io::{BufReader, Read};

fn main() {
    let stdin = std::io::stdin();

    let mut input = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_to_string(&mut input).unwrap();

    let val = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let (_id_part, data_part) = line.split_once(':').unwrap();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for set in data_part.split(';') {
                for datum in set.split(',') {
                    let (n, color) = datum.trim().split_once(' ').unwrap();
                    let n: u32 = n.parse().unwrap();

                    match color {
                        "red" if red < n => red = n,
                        "green" if green < n => green = n,
                        "blue" if blue < n => blue = n,
                        _ => {}
                    }
                }
            }

            Some(red * green * blue)
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Sum value: {val}");
}
