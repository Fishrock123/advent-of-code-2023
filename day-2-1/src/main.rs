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

            let (id_part, data_part) = line.split_once(':').unwrap();
            let id: u32 = id_part[5..].trim().parse().unwrap();

            data_part
                .split(';')
                .all(|set| {
                    set.split(',').all(|datum| {
                        let (n, color) = datum.trim().split_once(' ').unwrap();
                        let n: u32 = n.parse().unwrap();

                        match color {
                            "red" if n <= 12 => true,
                            "green" if n <= 13 => true,
                            "blue" if n <= 14 => true,
                            _ => false,
                        }
                    })
                })
                .then_some(id)
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Sum value: {val}");
}
