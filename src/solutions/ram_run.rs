use std::collections::VecDeque;

pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut iter = raw_data.lines();

    let size = iter.nth(0).and_then(
            |x| x.parse::<usize>().ok()
        ).unwrap() + 1;

    let fallen_bytes = iter.nth(0).and_then(|x| x.parse().ok()).unwrap();

    let byte_list: Vec<Vec<usize>> = iter.map(|l| 
            l.split(',').map(|x| x.parse().unwrap()).collect()
        )
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", min_steps_after_kb(&byte_list[..fallen_bytes], size));
    }

    if ignore != 2 {
        println!("Part 2: {:?}", first_block(&byte_list, fallen_bytes, size));
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
struct Bot {
    loc: (usize, usize),
    size: usize,
}

impl Bot {
    fn next_locs(&self) -> Vec<(usize, usize)> {
        let mut locs = Vec::new();

        for (dy, dx) in DIRECTIONS {
            let mut ni = self.loc.0; let mut nj = self.loc.1;
            if dy == -1 {
                if ni == 0 {
                    continue;
                } else {
                    ni = ni - 1;
                }
            } else if dy == 1 {
                if ni + 1 == self.size {
                    continue;
                } else {
                    ni = ni + 1;
                }
            }

            if dx == -1 {
                if nj == 0 {
                    continue;
                } else {
                    nj = nj - 1;
                }
            } else if dx == 1 {
                if nj + 1 == self.size {
                    continue;
                } else {
                    nj = nj + 1;
                }
            }

            locs.push((ni, nj));
        }

        return locs;
    }
}

fn min_steps_after_kb(byte_list: &[Vec<usize>], size: usize) -> i32 {
    let mut scores: Vec<Vec<i32>> = vec![vec![i32::MAX; size]; size];

    let startloc = (0, 0);
    let endloc = (size - 1, size - 1);
    scores[startloc.0][startloc.1] = 0;

    for r in byte_list {
        scores[r[1]][r[0]] = -1; // ie corrupted
    }

    let mut queue = VecDeque::new();
    queue.push_back(Bot { loc: startloc, size });

    while let Some(b) = queue.pop_front() {
        for (ni, nj) in b.next_locs() {
            if scores[ni][nj] == -1 {
                continue;
            }

            let temp = scores[b.loc.0][b.loc.1] + 1;
            if temp < scores[ni][nj] {
                scores[ni][nj] = temp;
                queue.push_back(Bot { loc: (ni, nj), size });
            }
        }
    }
    
    return scores[endloc.0][endloc.1];
}

fn first_block(byte_list: &Vec<Vec<usize>>, fallen_bytes: usize, size: usize) -> Vec<usize> {
    for i in fallen_bytes..byte_list.len() {
        if min_steps_after_kb(&byte_list[..i], size) > 10000 {
            return byte_list[i - 1].clone();
        }
    }
    
    return byte_list[0].clone();
}