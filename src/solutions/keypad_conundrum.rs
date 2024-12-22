use std::{collections::HashMap, i64};

pub fn run(filename: String, ignore: i32) {
    let codes = std::fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", codes.lines().map(|x| get_complexity(x, 2)).sum::<i64>());
    }

    if ignore != 2 {
        println!("Part 2: {}", codes.lines().map(|x| get_complexity(x, 25)).sum::<i64>());
    }
}

struct Robot {
    keys: HashMap<char, [i32; 2]>,
    forbidden: [i32; 2]
}

impl Robot {
    fn move_to(&self, src_char: char, dest_char: char) -> Vec<String> {
        let mut paths = Vec::new();

        let mut src = self.keys[&src_char].clone();
        let dest = self.keys[&dest_char];

        // First UD then RL
        macro_rules! gen_path {
            ($d: expr; $temp: ident; $valid: ident; $c1: literal, $c2: literal) => {
                while src[$d] != dest[$d] {
                    let mv = (dest[$d] - src[$d]).signum();
                    src[$d] += mv;
                    if src == self.forbidden {
                        $valid = false;
                        break;
                    }

                    if mv == 1 {
                        $temp.push($c1);
                    } else {
                        $temp.push($c2);
                    }
                }
            };
        }

        let mut temp = String::new();
        let mut valid = true;
        gen_path!(0; temp; valid; 'v', '^');
        gen_path!(1; temp; valid; '>', '<');
        temp.push('A');

        if valid {
            paths.push(temp);
        }

        let mut temp = String::new();
        let mut valid = true;
        src = self.keys[&src_char].clone();
        gen_path!(1; temp; valid; '>', '<');
        gen_path!(0; temp; valid; 'v', '^');
        temp.push('A');

        if valid && (paths.is_empty() || paths[0] != temp) {
            paths.push(temp);
        }
        
        return paths;
    }
}

/* fn aux(bot: &Robot, codes: Vec<String>) -> Vec<String> {
    let mut min_paths: Vec<String> = Vec::new();
    let mut prev = 'A';
    for to_type in codes {
        let mut options_for_curr_path = vec![String::new()];
        for c in to_type.chars() {
            let mut temp = Vec::new();
            for options in bot.move_to(prev, c) {
                for path in options_for_curr_path.iter() {
                    temp.push( 
                        path.to_string() + &options[..]
                    );
                }
            }
            options_for_curr_path = temp;
            prev = c;
        }
        
        for options in options_for_curr_path {
            if min_paths.is_empty() || min_paths[0].len() > options.len() {
                min_paths.clear();
                min_paths.push(options);
            } else if min_paths[0].len() == options.len() {
                min_paths.push(options);
            }
        }
    }

    return min_paths;
} */

fn get_complexity(code: &str, botnum: i32) -> i64 {
    let numeric_bot = Robot { 
        forbidden: [3, 0],
        keys: HashMap::from([
            ('A', [3, 2]),
            ('0', [3, 1]),
            ('3', [2, 2]),
            ('2', [2, 1]),
            ('1', [2, 0]),
            ('6', [1, 2]),
            ('5', [1, 1]),
            ('4', [1, 0]),
            ('9', [0, 2]),
            ('8', [0, 1]),
            ('7', [0, 0])
        ])
    };

    let direction_bot = Robot { 
        forbidden: [0, 0],
        keys: HashMap::from([
            ('A', [0, 2]),
            ('^', [0, 1]),
            ('>', [1, 2]),
            ('v', [1, 1]),
            ('<', [1, 0])
        ])
    };

    let mut numeric_paths = vec![String::new()];
    let mut prev = 'A';
    for c in code.chars() {
        let mut temp = Vec::new();
        for options in numeric_bot.move_to(prev, c) {
            for path in numeric_paths.iter() {
                temp.push( 
                    path.to_string() + &options[..]
                );
            }
        }
        numeric_paths = temp;
        prev = c;
    }

    let mut costs = HashMap::new();
    let mut button_combination_paths = HashMap::new();
    for start in direction_bot.keys.keys() {
        for end in direction_bot.keys.keys() {
            let temp = direction_bot.move_to(*start, *end);
            costs.insert((*start, *end), temp[0].len() as i64);
            button_combination_paths.insert((*start, *end), temp);
        }
    }

    for _ in 0..botnum-1 {
        let mut temp_costs = HashMap::new();

        for start in direction_bot.keys.keys() {
            for end in direction_bot.keys.keys() {
                let min_cost = button_combination_paths[&(*start, *end)].iter()
                    .map(|x| x.chars()
                        .fold((0, 'A'), |(c, prev), curr| (c + costs[&(prev, curr)], curr)).0
                    )
                    .min()
                    .unwrap();
                temp_costs.insert((*start, *end), min_cost);
            }
        }

        costs = temp_costs;
    }

    let len = numeric_paths.into_iter()
        .map(
            |x| x.chars()
            .fold((0, 'A'), |(c, prev), curr| (c + costs[&(prev, curr)], curr)).0
        )
        .min()
        .unwrap();

    return code.trim_end_matches('A').parse::<i64>().unwrap() * len;
}