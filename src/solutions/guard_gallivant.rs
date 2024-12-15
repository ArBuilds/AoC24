pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let data: Vec<Vec<char>> = raw_data.lines().map(|l| l.chars().collect()).collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!("Part 1: {}", count_locs(&data, rows, cols));
    }

    if ignore != 2 {
        println!("Part 2: {}", find_loop_locs(data, rows, cols));
    }
}

struct Bot {
    loc: (usize, usize),
    direction: usize,
    // 1 => up, 2 => right, 3 => down, 0 => left

    rows: usize,
    cols: usize
}

impl Bot {
    fn next_loc(&self) -> Option<(usize, usize)> {
        let mut ni = self.loc.0; let mut nj = self.loc.1;

        match self.direction {
            1 => if ni == 0 { return None; } else { ni = ni - 1; }
            2 => if nj + 1 == self.cols { return None; } else { nj = nj + 1; }
            3 => if ni + 1 == self.rows { return None; } else { ni = ni + 1; }
            0 => if nj == 0 { return None; } else { nj = nj - 1; }

            _ => { panic!("Invalid direction {}", self.direction); }
        }

        return Some((ni, nj));
    }

    fn advance_to(&mut self, i: usize, j: usize, d: usize) {
        self.loc = (i, j);
        self.direction = d;
    }
}

fn count_locs(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut guard = Bot { loc: (0, 0), direction: 1, rows, cols };
    let mut seen: Vec<Vec<bool>> = (0..rows).map( |_| (0..cols).map(|_| false).collect() ).collect();
    let mut count = 0;

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '^' { 
                guard.loc = (i, j); 
                seen[i][j] = true;
                count = 1;
                break;
            }
        }
    }

    while let Some((i, j)) = guard.next_loc() {
        if data[i][j] == '#' {
            guard.advance_to(guard.loc.0, guard.loc.1, (guard.direction + 1) % 4);
        }
        else {
            guard.advance_to(i, j, guard.direction);
            if !seen[i][j] {
                seen[i][j] = true;
                count += 1;
            }
        }
    }

    count
}

fn is_loop(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> bool {
    let mut guard = Bot { loc: (0, 0), direction: 1, rows, cols };
    let mut seen: Vec<Vec<[bool; 4]>> = (0..rows).map( |_| (0..cols).map(|_| [false, false, false, false]).collect() ).collect();

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '^' { 
                guard.loc = (i, j); 
                seen[i][j] = [false, true, false, false];
                break;
            }
        }
    }

    while let Some((i, j)) = guard.next_loc() {
        if data[i][j] == '#' {
            guard.advance_to(guard.loc.0, guard.loc.1, (guard.direction + 1) % 4);
        } else {
            guard.advance_to(i, j, guard.direction);  
        }
        
        if *seen[i][j].get(guard.direction).expect("Direction will be valid") {
            return true;
        } else {
            *seen[i][j].get_mut(guard.direction).expect("Direction will be valid") = true;
        }
    }

    false
}

fn find_loop_locs(mut data: Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    let mut count = 0;

    let mut guard = Bot { loc: (0, 0), direction: 1, rows, cols };
    let mut start = (0, 0);
    let mut inital_path: Vec<Vec<bool>> = (0..rows).map( |_| (0..cols).map(|_| false).collect() ).collect();

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == '^' { 
                guard.loc = (i, j); 
                start = guard.loc;
                inital_path[i][j] = true;
                break;
            }
        }
    }

    while let Some((i, j)) = guard.next_loc() {
        if data[i][j] == '#' {
            guard.advance_to(guard.loc.0, guard.loc.1, (guard.direction + 1) % 4);
        }
        else {
            guard.advance_to(i, j, guard.direction);
            if !inital_path[i][j] {
                inital_path[i][j] = true;
            }
        }
    }

    for (i, r) in inital_path.into_iter().enumerate() {
        for (j, in_path) in r.into_iter().enumerate() {
            if in_path && (start.0 != i || start.1 != j) { 
                data[i][j] = '#';
                if is_loop(&data, rows, cols) {
                    count += 1;
                }
                data[i][j] = '.'
            }
        }
    }

    count
}