use std::collections::{HashMap, VecDeque};

pub fn run(filename: String, ignore: i32) {
    let data: Vec<i64> = std::fs::read_to_string(filename).unwrap()
        .lines().map(|x| x.parse().unwrap()).collect();

    if ignore != 1 {
        println!("Part 1: {}", data.iter().map(|x| get_secret(*x, 2000)).sum::<i64>());
    }

    if ignore != 2 {
        println!("Part 2: {}", buy_bananas(data, 2000));
    }
}

fn get_secret(x: i64, number: i32) -> i64 {
    let mut secret = x;
    for _ in 0..number {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
    }

    secret
}

fn buy_bananas(data: Vec<i64>, number: i32) -> i64 {
    let mut prices: Vec<Vec<i64>> = data.iter().map(|x| vec![x % 10]).collect();

    for (i, x) in data.iter().enumerate() {
        let mut secret = *x;
        for _ in 0..number {
            secret = ((secret * 64) ^ secret) % 16777216;
            secret = ((secret / 32) ^ secret) % 16777216;
            secret = ((secret * 2048) ^ secret) % 16777216;

            prices[i].push(secret % 10);
        }
    }

    let mut changes = HashMap::new();
    for p in prices {
        let mut current_changes = VecDeque::new();
        for j in 1..4 {
            current_changes.push_back(p[j] - p[j - 1]);
        }

        let mut temp = HashMap::new();
        for j in 4..p.len() {
            current_changes.push_back(p[j] - p[j - 1]);
            if let None = temp.get_mut(&current_changes) {
                temp.insert(current_changes.clone(), p[j]);
            }
            current_changes.pop_front();
        }

        for (k, v) in temp {
            if let Some(x) = changes.get_mut(&k) {
                *x += v;
            } else {
                changes.insert(k, v);
            }
        }
    }
 
    return changes.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1) )
        .unwrap().1;
}