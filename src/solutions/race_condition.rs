use std::collections::VecDeque;

pub fn run(filename: String, ignore: i32) {
    let map: Vec<Vec<char>> = std::fs::read_to_string(filename).unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let rows = map.len(); let cols = map[0].len();

    if ignore != 1 {
        println!("Part 1: {}", find_cheats(&map, 2, 100, rows, cols));
    }

    if ignore != 2 {
        println!("Part 2: {}", find_cheats(&map, 20, 100, rows, cols));
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Bot {
    loc: (usize, usize),
    rows: usize,
    cols: usize
}

impl Bot {
    fn next_loc_specific(&self, step: usize, d: (i32, i32)) -> Option<(usize, usize)> {
        let (dy, dx) = d;
        let mut ni = self.loc.0; let mut nj = self.loc.1;
        if dy == -1 {
            if ni < step {
                return None;
            } else {
                ni = ni - step;
            }

        } else if dy == 1 {
            if ni + step >= self.rows {
                return None;
            } else {
                ni = ni + step;
            }

        }

        if dx == -1 {
            if nj < step {
                return None;
            } else {
                nj = nj - step;
            }

        } else if dx == 1 {
            if nj + step >= self.cols {
                return None;
            } else {
                nj = nj + step;
            }

        }

        return Some((ni, nj));
    }

    fn next_adj_locs(&self) -> Vec<(usize, usize, (i32, i32))> {
        let mut locs = Vec::new();

        for d in DIRECTIONS {
            if let Some((ni, nj)) = self.next_loc_specific(1, d) {
                locs.push((ni, nj, d));
            }
        }

        return locs;
    }
}

fn find_cheats(map: &Vec<Vec<char>>, cheat_time: usize, limit: usize, rows: usize, cols: usize) -> i32 {
    let mut times = vec![vec![0; cols]; rows];
    let mut seen = vec![vec![false; cols]; rows];

    let mut startloc = (0, 0); let mut endloc = (0, 0);
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'S' {
                startloc = (i, j);
                times[i][j] = 0;
                seen[i][j] = true;
            } else if *c == 'E' {
                endloc = (i, j);
            }
        }
    }

    if startloc == (0, 0) || endloc == (0, 0) {
        panic!("Couldn't find start and/or end");
    }

    let mut queue = VecDeque::new();
    queue.push_back(Bot { loc: startloc, rows, cols });

    while let Some(b) = queue.pop_front() {
        for (i, j, _) in b.next_adj_locs() {
            if map[i][j] != '#' && !seen[i][j] {
                times[i][j] = times[b.loc.0][b.loc.1] + 1;
                seen[i][j] = true;
                queue.push_back(Bot { loc: (i, j), rows, cols });
                break; // only one track
            }
        }
    }

    let mut count = 0;
    queue.push_back(Bot { loc: startloc, rows, cols });
    seen.fill(vec![false; cols]);
    seen[startloc.0][startloc.1] = true;

    while let Some(b) = queue.pop_front() {
        for i in 0..rows {
            for j in 0..cols {
                let temp = i.abs_diff(b.loc.0) + j.abs_diff(b.loc.1);
                if temp <= cheat_time {
                    if map[i][j] != '#' {
                        if times[i][j] >= limit + temp + times[b.loc.0][b.loc.1] {
                            count += 1;
                        }

                    }
                }
            }
        }

        for (i, j, _) in b.next_adj_locs() {
            if map[i][j] != '#' && !seen[i][j] {
                seen[i][j] = true;
                queue.push_back(Bot { loc: (i, j), rows, cols });
                break; // only one track
            }
        }
    }

    return count;
}