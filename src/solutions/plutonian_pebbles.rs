use std::collections::HashMap;

pub fn run(filename: String, ignore: i32) {
    let data: Vec<(i64, u32)> = std::fs::read_to_string(filename).unwrap()
        .split_whitespace()
        .map(|x| (x.parse().unwrap(), x.len().try_into().unwrap()))
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", count_stones_large(&data, 25));
    }

    if ignore != 2 {
        println!("Part 2: {}", count_stones_large(&data, 75));
    }
}

fn get_digit_number(n: i64, guess: u32) -> u32 {
    if n < 10 {
        return 1;
    }

    let mut power = 10_i64.pow(guess);
    let mut d = guess;
    while n / power == 0 {
        power /= 10; d -= 1;
    }

    return d + 1;
}

fn count_stones_large(stones: &Vec<(i64, u32)>, rounds: i32) -> i64 {
    let mut data: HashMap<i64, (i64, u32)> = HashMap::new();
    stones.iter().for_each(|x| { data.insert((*x).0, (1, (*x).1)); });

    for _blink in 0..rounds {
        let mut newdata: HashMap<i64, (i64, u32)> = HashMap::new();

        for (n, (count, d)) in data.iter() {
            if *n == 0 {
                if let Some(x) = newdata.get_mut(&1) {
                    (*x).0 += *count;
                } else {
                    newdata.insert(1, (*count, 1));
                }

            } else if *d % 2 == 0 {
                let halfdigits = *d / 2;
                let power: i64 = 10_i64.pow(halfdigits);

                let a = *n / power;
                if let Some(x) = newdata.get_mut(&a) {
                    (*x).0 += *count;
                } else {
                    newdata.insert(a, (*count, halfdigits));
                }

                let b = *n % power;
                if let Some(x) = newdata.get_mut(&b) {
                    (*x).0 += *count;
                } else {
                    newdata.insert(b, (*count, get_digit_number(b, halfdigits)));
                }

            } else {
                let a = *n * 2024;
                if let Some(x) = newdata.get_mut(&a) {
                    (*x).0 += *count;
                } else {
                    newdata.insert(a, (*count, get_digit_number(a, *d + 4)));
                }

            }
        }

        data = newdata;
    }

    return data.into_values().map(|(n, _)| n).sum();
}