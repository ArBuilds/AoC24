pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let data: Vec<Vec<char>> = raw_data.lines()
        .map(|c| c.chars().collect())
        .collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!("Part 1: {}", 
            find_xmas(&data, rows, cols)
        );
    }

    if ignore != 2 {
        println!("Part 2: {}", 
            find_x_mas(&data, rows, cols)
        );
    }
}

struct Bot {
    symbol: char,
    loc: (usize, usize),
    direction: (i32, i32),
    rows: usize,
    cols: usize
}

impl Bot {
    fn next_loc(&self) -> Option<(char, usize, usize)> {
        let mut ni = self.loc.0; let mut nj = self.loc.1;

        if self.direction.0 == -1 {
            if ni == 0 {
                return None;
            } else {
                ni = ni - 1;
            }
        } else if self.direction.0 == 1 {
            if ni + 1 == self.rows {
                return None;
            } else {
                ni = ni + 1;
            }
        }

        if self.direction.1 == -1 {
            if nj == 0 {
                return None;
            } else {
                nj = nj - 1;
            }
        } else if self.direction.1 == 1 {
            if nj + 1 == self.cols {
                return None;
            } else {
                nj = nj + 1;
            }
        }

        let nc = match self.symbol {
            'X' => 'M', 'M' => 'A', 'A' => 'S', 'S' => '.', _ =>  { return None; }
        };

        return Some((nc, ni, nj));
    }

    fn advance_to(&mut self, c: char, i: usize, j: usize) {
        self.loc = (i, j);
        self.symbol = c;
    }
}

fn find_xmas(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut counter = 0;
    for i in 0..rows {
        for j in 0..cols {
            if data[i][j] == 'X' {
                for direction in [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)] {
                    let mut b = Bot { symbol: 'X', loc: (i, j), direction, rows, cols };
                    while let Some((nc, ni, nj)) = b.next_loc() {
                        if data[ni][nj] == nc {
                            if nc == 'S'  {
                                counter += 1;
                                break;
                            }
                            b.advance_to(nc, ni, nj);
                        } else {
                            break;
                        }
                    }
                }

                // println!("{i}, {j}: {counter}");
            }
        }
    }

    return counter;
}

fn find_x_mas(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut counter = 0;
    for i in 0..rows {
        for j in 0..cols {
            if data[i][j] == 'A' {
                let mut prev = '.'; let mut mcount = 0; let mut scount = 0;

                for (n, c) in [(-1, -1), (1, 1), (1, -1), (-1, 1)].into_iter().map(
                    |direction| {
                        let b = Bot { symbol: 'X', loc: (i, j), direction, rows, cols };
                        if let Some((_, ni, nj)) = b.next_loc() {
                            data[ni][nj]
                        } else {
                            '.'
                        }
                    }
                ).enumerate() {

                    if c == prev && n % 2 == 1 { // 1 should not be same as 0, 3 should not be same as 2
                        break;
                    }

                    prev = c;
                    if c == 'M' {
                        mcount += 1;
                    } else if c == 'S' {
                        scount += 1;
                    } else {
                        break;
                    }
                }

                if mcount == 2 && scount == 2 {
                    counter += 1;
                }
            }
        }
    }

    return counter;
}