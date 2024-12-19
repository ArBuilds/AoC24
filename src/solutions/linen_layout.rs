use std::collections::HashMap;
pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut lines = raw_data.lines();

    let mut available_patterns: HashMap<char, Vec<Vec<char>>> = HashMap::from_iter(
        "wubrg".chars().map(|c| (c, vec![]))
    );
    
    for pat in lines.nth(0).unwrap().split(", ").map(|p| p.chars().collect::<Vec<char>>()) {
        if let Some(v) = available_patterns.get_mut(&pat[0]) {
            (*v).push(pat);
        }
    }

    lines.next();
    let designs: Vec<i64> = lines.map(|s| s.chars().collect())
        .map(|x| count_designs(x, &available_patterns))
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", designs.iter().filter(|x| **x > 0).count());
    }

    if ignore != 2 {
        println!("Part 2: {}", designs.iter().sum::<i64>());
    }
}

fn count_designs(pat: Vec<char>, available_patterns: &HashMap<char, Vec<Vec<char>>>) -> i64 {
    let l = pat.len();
    let mut dp = vec![0; l + 1];
    dp[l] = 1; 

    for i in (0..l).rev() {
        if let Some(values) = available_patterns.get(&pat[i]) {
            for v in values {
                if v.len() > l - i {
                    continue;
                }

                let mut is_match = true;
                for j in 0..v.len() {
                    if v[j] != pat[i + j] {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    dp[i] += dp[i + v.len()];
                }
            }

        }
    }

    return dp[0];
}