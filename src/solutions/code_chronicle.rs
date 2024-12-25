use std::iter::zip;

pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut is_lock = false;
    for (i, l) in raw_data.lines().enumerate() {
        if i % 8 == 0 {
            is_lock = l == "#####";
            
            // -1 to account for top and bottom rows
            if is_lock {
                locks.push(vec![-1; 5]);
            } else {
                keys.push(vec![-1; 5]);
            }
        }

        let curr = if is_lock { locks.len() - 1 } else { keys.len() - 1 };
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                if is_lock {
                    locks[curr][j] += 1;
                } else {
                    keys[curr][j] += 1;
                }
            }
        }
    }

    if ignore != 1 {
        println!("Part 1: {}", find_non_overlapping(&locks, &keys));
    }
}

fn find_non_overlapping(locks: &Vec<Vec<i32>>, keys: &Vec<Vec<i32>>) -> i64 {
    let mut count = 0;
    
    for l in locks {
        for k in keys {
            let mut fits = true;
            for (a, b) in zip(l, k) {
                if a + b > 5 {
                    fits = false;
                    break;
                }
            }

            if fits {
                count += 1;
            }
        }
    }

    return count;
}