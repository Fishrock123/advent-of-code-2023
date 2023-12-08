use std::io::{BufReader, Read};

fn main() {
    let stdin = std::io::stdin();

    let mut input = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_to_string(&mut input).unwrap();

    let val = input
        .split('\n')
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit());
            let last = line.chars().rev().find(|c| c.is_ascii_digit());

            let (Some(first), Some(last)) = (first, last) else {
                return 0;
            };

            let sum: u32 = format!("{first}{last}").parse().unwrap();

            println!("{line} > {first} + {last} = {sum}");

            sum
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Sum value: {val}");
}
