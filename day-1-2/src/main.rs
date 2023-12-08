use std::io::{BufReader, Read};

fn main() {
    let stdin = std::io::stdin();

    let mut input = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_to_string(&mut input).unwrap();

    let val = input
        .split('\n')
        .map(|line| {
            let first = find_digit(line.chars(), false);
            let last = find_digit(line.chars().rev(), true);

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

fn find_digit(iter: impl Iterator<Item = char>, rev: bool) -> Option<u32> {
    let mut digit = None;
    let mut digit_str = String::new();

    for c in iter {
        if let Some(n) = c.to_digit(10) {
            digit = Some(n);
            break;
        }

        if rev {
            digit_str.insert(0, c)
        } else {
            digit_str.push(c);
        }

        if let Some(n) = string_to_digit(&digit_str) {
            digit = Some(n);
            break;
        }
    }

    digit
}

fn string_to_digit(input: &str) -> Option<u32> {
    match input {
        s if s.contains("zero") => Some(0),
        s if s.contains("one") => Some(1),
        s if s.contains("two") => Some(2),
        s if s.contains("three") => Some(3),
        s if s.contains("four") => Some(4),
        s if s.contains("five") => Some(5),
        s if s.contains("six") => Some(6),
        s if s.contains("seven") => Some(7),
        s if s.contains("eight") => Some(8),
        s if s.contains("nine") => Some(9),
        _ => None,
    }
}
