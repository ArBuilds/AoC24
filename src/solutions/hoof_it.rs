pub fn run(filename: String, ignore: i32) {
    let data: Vec<Vec<char>> = std::fs::read_to_string(filename).unwrap()
        .lines().map(|l| l.chars().collect())
        .collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!{"Part 1: {}", get_trail_scores(&data, rows, cols)}
    }

    if ignore != 2 {
        println!{"Part 2: {}", get_trail_ratings(&data, rows, cols)}
    }
}

struct Bot {
    symbol: char,
    loc: (usize, usize),
    rows: usize,
    cols: usize
}

impl Bot {
    fn next_valid_locs(&self) -> Vec<(char, usize, usize)> {
        let mut locs = Vec::new();
        
        for d in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let mut ni = self.loc.0; let mut nj = self.loc.1;

            if d.0 == -1 {
                if ni == 0 {
                    continue;
                } else {
                    ni = ni - 1;
                }
            } else if d.0 == 1 {
                if ni + 1 == self.rows {
                    continue;
                } else {
                    ni = ni + 1;
                }
            }

            if d.1 == -1 {
                if nj == 0 {
                    continue;
                } else {
                    nj = nj - 1;
                }
            } else if d.1 == 1 {
                if nj + 1 == self.cols {
                    continue;
                } else {
                    nj = nj + 1;
                }
            }

            let nc_op = self.symbol.to_digit(10)
                .and_then(|n| char::from_digit(n + 1, 10));

            if let Some(nc) = nc_op {
                locs.push((nc, ni, nj));
            }
        }

        return locs;
    }
}

fn get_trail_scores(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    use std::collections::VecDeque;
    let mut score = 0;

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '0' {
                let mut bot_queue: VecDeque<_> = VecDeque::from([ ('0', i, j) ]);

                let mut seen = Vec::new();

                while !bot_queue.is_empty() {
                    let b = bot_queue[0];
                    let bot = Bot { symbol: b.0, loc: (b.1, b.2), rows, cols };
                    for (nc, ni, nj) in bot.next_valid_locs() {
                        if data[ni][nj] == nc {
                            if seen.contains(&(ni, nj)) {
                                continue;
                            }

                            seen.push((ni, nj));
                            if nc == '9' {
                                score += 1;
                            } else {
                                bot_queue.push_back((nc, ni, nj));
                            }
                        }
                    }

                    bot_queue.pop_front();
                }
            }
        }
    }

    return score;
}

fn get_trail_ratings(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    use std::collections::VecDeque;
    let mut rating = 0;

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '0' {
                let mut bot_queue: VecDeque<_> = VecDeque::from([ ('0', i, j) ]);

                while !bot_queue.is_empty() {
                    let b = bot_queue[0];
                    let bot = Bot { symbol: b.0, loc: (b.1, b.2), rows, cols };
                    for (nc, ni, nj) in bot.next_valid_locs() {
                        if data[ni][nj] == nc {
                            if nc == '9' {
                                rating += 1;
                            } else {
                                bot_queue.push_back((nc, ni, nj));
                            }
                        }
                    }

                    bot_queue.pop_front();
                }
            }
        }
    }

    return rating;
}