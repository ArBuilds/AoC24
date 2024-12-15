pub fn run(filename: String, ignore: i32) {
    use regex::Regex;

    let raw_data = std::fs::read_to_string(filename).unwrap();
    let re = Regex::new("[0-9]+").unwrap();

    let mut data: Vec<Vec<i64>> = Vec::new();
    for (i, m) in re.find_iter(&raw_data).enumerate() {
        if i % 6 == 0 {
            data.push(vec![]);
        }
        data[i / 6].push(m.as_str().parse().unwrap());
    }

    if ignore != 1 {
        println!("Part 1: {}", data.iter().map(|x| spent_tokens(x)).sum::<i64>());
    }

    if ignore != 2 {
        println!("Part 2: {}", data.iter().map(|x| spent_tokens_large(x)).sum::<i64>());
    }
}

fn spent_tokens(data: &Vec<i64>) -> i64 {
    let n_b = data[0] * data[5] - data[1] * data[4];
    let n_a = data[3] * data[4] - data[2] * data[5];
    let d = data[0] * data[3] - data[1] * data[2];

    if n_a % d == 0 && n_b % d == 0 {
        (3 * n_a + n_b) / d
    } else {
        0
    }
}

fn spent_tokens_large(data: &Vec<i64>) -> i64 {
    let n_b = data[0] * data[5] - data[1] * data[4] + 10000000000000 * (data[0] - data[1]);
    let n_a = data[3] * data[4] - data[2] * data[5] + 10000000000000 * (data[3] - data[2]);
    let d = data[0] * data[3] - data[1] * data[2];

    if n_a % d == 0 && n_b % d == 0 {
        (3 * n_a + n_b) / d
    } else {
        0
    }
}