use std::{collections::VecDeque, i32};

pub fn run(filename: String, ignore: i32) {
    let data: Vec<Vec<char>> = std::fs::read_to_string(filename).unwrap()
        .lines().map(|l| l.chars().collect()).collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!("Part 1: {}", solve_maze(&data, rows, cols));
    }

    if ignore != 2 {
        println!("Part 2: {}", find_seats(&data, rows, cols));
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
struct Bot {
    loc: (usize, usize),
    direction: usize,
    rows: usize,
    cols: usize,
    score: i32,
    visited: Vec<(usize, usize)>
}

impl Bot {
    fn next_locs(&self) -> Vec<((usize, usize), usize)> {
        let mut locs = Vec::new();

        for d in (self.direction + 3) ..= (self.direction + 5) {
            let mut ni = self.loc.0; let mut nj = self.loc.1;
            if DIRECTIONS[d % 4].0 == -1 {
                if ni == 0 {
                    continue;
                } else {
                    ni = ni - 1;
                }
            } else if DIRECTIONS[d % 4].0 == 1 {
                if ni + 1 == self.rows {
                    continue;
                } else {
                    ni = ni + 1;
                }
            }

            if DIRECTIONS[d % 4].1 == -1 {
                if nj == 0 {
                    continue;
                } else {
                    nj = nj - 1;
                }
            } else if DIRECTIONS[d % 4].1 == 1 {
                if nj + 1 == self.cols {
                    continue;
                } else {
                    nj = nj + 1;
                }
            }

            locs.push(((ni, nj), d % 4));
        }

        return locs;
    }
}

fn solve_maze(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut scores: Vec<Vec<i32>> = vec![vec![i32::MAX; cols]; rows];

    let mut startloc = (rows - 2, 1);
    if data[rows - 2][1] != 'S' {
        for i in 0..rows {
            for j in 0..cols {
                if data[i][j] == 'S' {
                    startloc = (i, j);
                }
            }
        }
    }

    let mut endloc = (1, cols - 2);
    if data[rows - 2][1] != 'E' {
        for i in 0..rows {
            for j in 0..cols {
                if data[i][j] == 'E' {
                    endloc = (i, j);
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Bot { loc: startloc, direction: 0, rows, cols, score: 0, visited: vec![] });
    scores[startloc.0][startloc.1] = 0;

    while let Some(b) = queue.pop_front() {
        for ((ni, nj), d) in b.next_locs() {
            if data[ni][nj] == '#' {
                continue;
            }

            let mut temp = b.score + 1;
            if d != b.direction {
                temp += 1000;
            }

            if temp < scores[ni][nj] {
                scores[ni][nj] = temp;
                queue.push_back(Bot { 
                    loc: (ni, nj), direction: d, rows, cols, 
                    score: temp, visited: vec![] // no use for visited here
                });
            }
        }
    }
    
    return scores[endloc.0][endloc.1];
}

fn find_seats(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut scores: Vec<Vec<[i32; 2]>> = vec![vec![[i32::MAX; 2]; cols]; rows];

    let mut startloc = (rows - 2, 1);
    if data[rows - 2][1] != 'S' {
        for i in 0..rows {
            for j in 0..cols {
                if data[i][j] == 'S' {
                    startloc = (i, j);
                }
            }
        }
    }

    let mut endloc = (1, cols - 2);
    if data[rows - 2][1] != 'E' {
        for i in 0..rows {
            for j in 0..cols {
                if data[i][j] == 'E' {
                    endloc = (i, j);
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Bot { loc: startloc, direction: 0, rows, cols, score: 0, visited: vec![(0, 0)] });
    scores[startloc.0][startloc.1] = [0, 0];

    let mut endscore = i32::MAX;
    let mut end_visited_vecs = vec![];
    while let Some(b) = queue.pop_front() {
        for ((ni, nj), d) in b.next_locs() {
            if data[ni][nj] == '#' {
                continue;
            }

            let mut temp = b.score + 1;
            if d != b.direction {
                temp += 1000;
            }
            
            if data[ni][nj] == 'E' {
                if temp < endscore {
                    endscore = temp;
                    end_visited_vecs = vec![vec![endloc], b.visited.clone()];
                } else if temp == endscore {
                    end_visited_vecs.push(b.visited.clone());
                }

            } else if temp <= scores[ni][nj][d % 2] {
                scores[ni][nj][d % 2] = temp;
                let mut visited = b.visited.clone();
                visited.push((ni, nj));
                queue.push_back(Bot { 
                    loc: (ni, nj), direction: d, rows, cols, 
                    score: temp, visited
                });

            }
        }
    }

    let mut seen = vec![vec![false; cols]; rows];
    let mut count = 0;
    for v in end_visited_vecs {
        for (i, j) in v {
            if !seen[i][j] {
                seen[i][j] = true;
                count += 1;
            }
        }
    }
    
    return count;
}