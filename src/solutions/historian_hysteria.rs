use std::fs;

pub fn run(filename: String, ignore: i32) {
    let data = fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", distance_after_sort(&data));
    }

    if ignore != 2 {
        println!("Part 2: {}", similarity(&data));
    }
}

fn distance_after_sort(data: &String) -> i32 {
    use std::collections::BinaryHeap;

    let mut n = data.split_whitespace();
    let mut left = BinaryHeap::new(); let mut right = BinaryHeap::new();

    loop {
        if let Some(l) = n.next() {
            left.push(l.parse::<i32>().unwrap());
        } else {
            break;
        }

        if let Some(r) = n.next() {
            right.push(r.parse::<i32>().unwrap());
        } else {
            break;
        }
    }

    let mut sum = 0;
    loop {
        if let (Some(l), Some(r)) = (left.pop(), right.pop()) {
            if l < r {
                sum += r - l;
            } else {
                sum += l - r;
            }
        } else {
            break;
        }
    }

    return sum;
}

fn similarity(data: &String) -> i32 {
    use std::collections::HashMap;

    let mut n = data.split_whitespace();
    let mut left = vec![]; let mut right: HashMap<i32, i32> = HashMap::new();

    loop {
        if let Some(l) = n.next() {
            left.push(l.parse::<i32>().unwrap());
        } else {
            break;
        }

        if let Some(r) = n.next() {
            let temp = r.parse::<i32>().unwrap();
            if let Some(x) = right.get_mut(&temp) {
                *x += 1;
            } else {
                right.insert(temp, 1);
            }
        } else {
            break;
        }
    }

    let mut sum = 0;
    loop {
        if let Some(l) = left.pop() {
            if let Some(times) = right.get(&l) {
                sum += l * *times;
            }
        } else {
            break;
        }
    }

    return sum;
}