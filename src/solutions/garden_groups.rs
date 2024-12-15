pub fn run(filename: String, ignore: i32) {
    let data: Vec<Vec<char>> = std::fs::read_to_string(filename).unwrap()
        .lines().map(|l| l.chars().collect())
        .collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!("Part 1: {}", quote_fencing_price(&data, rows, cols));
    }

    if ignore != 2 {
        println!("Part 2: {}", quote_fencing_price_with_discount(&data, rows, cols));
    }
}

struct Bot {
    symbol: char,
    loc: (usize, usize),
    rows: usize,
    cols: usize
}

impl Bot {
    fn next_valid_locs(&self) -> Vec<(usize, usize, usize)> {
        let mut locs = Vec::new();
        
        // 0 = go down, 1 = go right, 2 = go up, 3 = go left
        for (i, d) in [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter().enumerate() {
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

            locs.push((ni, nj, i));
        }

        return locs;
    }
}

fn quote_fencing_price(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i64 {
    use std::collections::VecDeque;

    let mut region_id: Vec<Vec<(usize, i64)>> = vec![vec![(0, 0); cols]; rows];

    let mut rcount = 0;
    for i in 0..rows {
        for j in 0..cols {
            if region_id[i][j].0 == 0 {
                rcount += 1;
                region_id[i][j].0 = rcount;

                let mut bot_queue: VecDeque<_> = VecDeque::from([ (data[i][j], i, j) ]);
                while !bot_queue.is_empty() {
                    let b = bot_queue[0];
                    let bot = Bot { symbol: b.0, loc: (b.1, b.2), rows, cols };
                    for (ni, nj, _) in bot.next_valid_locs() {
                        if data[ni][nj] == bot.symbol {
                            region_id[b.1][b.2].1 += 1;

                            if region_id[ni][nj].0 == 0 {
                                region_id[ni][nj].0 = rcount;
                                bot_queue.push_back((bot.symbol, ni, nj));
                            } else if region_id[ni][nj].0 != rcount {
                                panic!("Uh oh {} != {rcount}", region_id[ni][nj].0);
                            }
                        }
                    }

                    bot_queue.pop_front();
                }
            }
        }
    }

    let mut regions = vec![(0, 0); rcount + 1];
    for i in 0..rows {
        for j in 0..cols {
            let (rid, adj) = region_id[i][j];
            regions[rid].0 += 1; regions[rid].1 += 4 - adj;
        }
    }

    regions.into_iter().map(|(a, p)| a * p).sum()
}

fn quote_fencing_price_with_discount(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i64 {
    use std::collections::VecDeque;

    let mut region_id: Vec<Vec<(usize, [bool; 4])>> = vec![vec![(0, [true; 4]); cols]; rows];

    let mut rcount = 0;
    for i in 0..rows {
        for j in 0..cols {
            if region_id[i][j].0 == 0 {
                rcount += 1;
                region_id[i][j].0 = rcount;

                let mut bot_queue: VecDeque<_> = VecDeque::from([ (data[i][j], i, j) ]);
                while !bot_queue.is_empty() {
                    let b = bot_queue[0];
                    let bot = Bot { symbol: b.0, loc: (b.1, b.2), rows, cols };
                    for (ni, nj, dir_id) in bot.next_valid_locs() {
                        // dir_id: 0 = down, 1 = right, 2 = up, 3 = left

                        if data[ni][nj] == bot.symbol {
                            region_id[b.1][b.2].1[dir_id] = false;

                            if region_id[ni][nj].0 == 0 {
                                region_id[ni][nj].0 = rcount;
                                bot_queue.push_back((bot.symbol, ni, nj));
                            } else if region_id[ni][nj].0 != rcount {
                                panic!("Uh oh {} != {rcount}", region_id[ni][nj].0);
                            }
                        }
                    }

                    bot_queue.pop_front();
                }
            }
        }
    }

    let mut regions = vec![(0, 0); rcount + 1];
    let mut prev: (usize, bool, bool);
    for i in 0..rows {
        let (rid, [a, _, b, _]) = region_id[i][0];
        regions[rid].0 += 1;

        if a {
            regions[rid].1 += 1;
        }

        if b {
            regions[rid].1 += 1;
        }

        prev = (rid, a, b);

        for j in 1..cols {
            let (rid, [a, _, b, _]) = region_id[i][j];
            regions[rid].0 += 1;
            
            if prev.0 == rid {
                if a && !prev.1 {
                    regions[rid].1 += 1;
                }

                if b && !prev.2 {
                    regions[rid].1 += 1;
                }

            } else {
                if a {
                    regions[rid].1 += 1;
                }
        
                if b {
                    regions[rid].1 += 1;
                }

            }

            prev = (rid, a, b);
        }
    }

    for j in 0..cols {
        let (rid, [_, a, _, b]) = region_id[0][j];

        if a {
            regions[rid].1 += 1;
        }

        if b {
            regions[rid].1 += 1;
        }

        prev = (rid, a, b);

        for i in 1..rows {
            let (rid, [_, a, _, b]) = region_id[i][j];
            
            if prev.0 == rid {
                if a && !prev.1 {
                    regions[rid].1 += 1;
                }

                if b && !prev.2 {
                    regions[rid].1 += 1;
                }

            } else {
                if a {
                    regions[rid].1 += 1;
                }
        
                if b {
                    regions[rid].1 += 1;
                }

            }

            prev = (rid, a, b);
        }
    }

    regions.into_iter().map(|(a, p)| a * p).sum()
}