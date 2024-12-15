use std::collections::HashMap;

pub fn run(filename: String, ignore: i32) {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut data_iter = data.lines().map(|x| x.to_string());

    let raw_config: Vec<Vec<i32>> = data_iter
        .by_ref()
        .map_while(|l| 
            if (*l).is_empty() { None } 
            else { Some ((*l).split('|').map(|x| x.parse::<i32>().unwrap()).collect()) }
        )
        .collect();

    let mut config: HashMap<i32, Vec<i32>> = HashMap::new();
    for values in raw_config {
        if values.len() != 2 {
            panic!("Error in data processing! Found {:?}", values);
        }

        if let Some(y) = config.get_mut(&values[0]) {
            (*y).push(values[1]);
        } else {
            config.insert(values[0], vec![values[1]]);
        }
    }

    let updates = data_iter
        .map(|update|
            update.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
        )
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", verify_updates(&config, &updates));
    }

    if ignore != 2 {
        println!("Part 2: {}", rectify_updates(&config, &updates));
    }
}

fn verify_updates(config: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    updates.into_iter()
        .filter_map(
            | update | {
                let mut seen = Vec::new();

                let middle = if update.len() % 2 == 0 {
                    panic!("Uh oh, found {:?}", update);
                } else { update[update.len() / 2] };

                for x in update {
                    if let Some(before_values) = config.get(x) {
                        if before_values.into_iter().any(|y| seen.contains(y)) {
                            return None;
                        }
                    }

                    seen.push(*x);
                }

                return Some( middle );
            }
        )
        .sum()
}

fn rectify_updates(config: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    updates.into_iter()
        .filter_map(
            | u | {
                let mut update = u.clone();

                let l = update.len();
                if l % 2 == 0 {
                    panic!("Even length update {:?}", update);
                }

                for i in 0..l*l {
                    let mut seen = HashMap::new();
                    let mut swapped = false; 

                    for j in 0..l {
                        let x = update[j];
                        if let Some(before_values) = config.get(&x) {
                            if let Some(k) = before_values.into_iter().find_map(|y| seen.get(y)) {
                                swapped = true;
                                update.swap(*k, j);
                            }
                        }

                        seen.insert(x, j);
                    }

                    println!("{:?}", update);

                    if !swapped {
                        if i == 0 {
                            return None;
                        }

                        return Some(update[l/2]);
                    }
                }

                panic!("Didn't work");
            }
        )
        .inspect(|x| println!("{}", *x))
        .sum()
}