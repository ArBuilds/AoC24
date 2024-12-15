use std::fs;

pub fn run(filename: String, ignore: i32) {
    let data = fs::read_to_string(filename).expect("Aborting run, could not read data.");
    
    if ignore != 1 {
        println!("Part 1: {}",
            data.lines().map(get_calibration_value_numeric).sum::<i32>()
        );
    }

    if ignore != 2 {
        println!("Part 2: {}",
            data.lines().map(get_calibration_value_alphanumeric).sum::<i32>()
        );
    }
}

fn get_calibration_value_numeric(line: &str) -> i32 {
    let mut numbers = line.matches(char::is_numeric)
        .map(|x| x.parse::<i32>().expect("Already verified digits"));

    let first = numbers.nth(0).expect("Line must have at least one digit");
    match numbers.last() {
        None => first * 11,
        Some(x) => first * 10 + x
    }
}

fn get_calibration_value_alphanumeric(line: &str) -> i32 {
    let mut first = (usize::MAX, 0); let mut last = (0, 0);
    let patterns = ["1", "one", "2", "two", "3", "three", "4", "four", 
        "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine"];

    for (i, n) in patterns.into_iter().enumerate() {
        if let Some(loc) = line.find(n) {
            if first.0 > loc { first = (loc, i / 2 + 1); }
            if last.0 <= loc { last = (loc, i / 2 + 1); }
        }
    }

    (first.1 * 10 + last.1).try_into().unwrap()
}