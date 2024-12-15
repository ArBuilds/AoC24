pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut iter = raw_data.lines();
    let grid: Vec<Vec<char>> = iter.by_ref().map_while(|x| if x.is_empty() { None } else { Some( x.chars().collect() ) }).collect();
    let moves: String = iter.map(|x| x.to_string()).collect();

    if ignore != 1 {
        println!("Part 1: {}", get_box_locs(grid.clone(), &moves));
    }

    if ignore != 2 {
        let mut wide_grid: Vec<Vec<char>> = Vec::new();
        for (i, r) in grid.into_iter().enumerate() {
            wide_grid.push(vec![]);
            for c in r {
                if c == 'O' {
                    wide_grid[i].push('['); wide_grid[i].push(']');
                } else {
                    wide_grid[i].push(c);
                    wide_grid[i].push(
                        match c {
                            '#' => '#',
                            _ => '.'
                        }
                    );
                }
            }
        }

        println!("Part 2: {}", get_box_locs(wide_grid, &moves));
    }
}

struct Bot {
    loc: (usize, usize),
    rows: usize,
    cols: usize
}

impl Bot {
    fn get_loc(&self, d: (i32, i32)) -> Result<(usize, usize), String> {
        let mut ni = self.loc.0; let mut nj = self.loc.1;

        if d.0 == -1 {
            if ni == 0 {
                return Err(format!("^ on top edge"));
            } else {
                ni = ni - 1;
            }
        } else if d.0 == 1 {
            if ni + 1 == self.rows {
                return Err(format!("v on bottom edge"));
            } else {
                ni = ni + 1;
            }
        }

        if d.1 == -1 {
            if nj == 0 {
                return Err(format!("< on left edge"));
            } else {
                nj = nj - 1;
            }
        } else if d.1 == 1 {
            if nj + 1 == self.cols {
                return Err(format!("> on right edge"));
            } else {
                nj = nj + 1;
            }
        }

        return Ok((ni, nj));
    }

    fn move_bot(&mut self, loc: (usize, usize)) {
        self.loc = loc;
    }
}

fn get_box_locs(mut grid: Vec<Vec<char>>, moves: &String) -> usize {
    let rows = grid.len(); let cols = grid[0].len();
    let mut robot = Bot { loc: (0, 0), rows, cols };
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                robot.loc = (i, j);
                grid[i][j] = '.';
                break;
            }
        }
    }

    for c in moves.chars() {
        let d = match c {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => { continue; }
        };
        
        let (x, y) = robot.get_loc(d).expect("Bounded by wall, not going to leave grid");
        if grid[x][y] == '.' {
            robot.move_bot((x, y));

        } else if grid[x][y] == 'O' {
            let mut boxbot = Bot { loc: (x, y), rows, cols };
            while let Ok((ni, nj)) = boxbot.get_loc(d) {
                if grid[ni][nj] == '.' {
                    grid[ni][nj] = 'O';
                    grid[x][y] = '.';
                    robot.move_bot((x, y));
                    break;
                } else if grid[ni][nj] == 'O' {
                    boxbot.move_bot((ni, nj));
                } else {
                    break;
                }
            }

        } else if grid[x][y] == '[' || grid[x][y] == ']' {
            if let Ok(g) = push_box(grid.clone(), rows, cols, (x, y), d) {
                grid = g;
                robot.move_bot((x, y));
            }
        }
    }

    let mut count = 0;
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                count += 100 * i + j;
            }
        }
    }
    for r in grid.iter() {
        for c in r {
            print!("{c}");
        }
        println!("");
    }
    return count;
}

fn push_box(mut grid: Vec<Vec<char>>, rows: usize, cols: usize, loc: (usize, usize), d: (i32, i32)) -> Result<Vec<Vec<char>>, String> {
    if grid[loc.0][loc.1] != '[' && grid[loc.0][loc.1] != ']' {
        return Ok(grid);
    }

    if d == (0, -1) {
        if loc.1 < 2 {
            return Err(format!("Cannot move left"));

        } else if grid[loc.0][loc.1 - 2] == '#' {
            return Err(format!("Wall crash"));

        } else {
            grid = push_box(grid, rows, cols, (loc.0, loc.1 - 2), d)?;

            grid[loc.0][loc.1 - 2] = '[';
            grid[loc.0][loc.1 - 1] = ']';
        }
    }

    if d == (0, 1) {
        if loc.1 + 2 >= cols {
            return Err(format!("Cannot move right"));
            
        } else if grid[loc.0][loc.1 + 2] == '#' {
            return Err(format!("Wall crash"));

        } else {
            grid = push_box(grid, rows, cols, (loc.0, loc.1 + 2), d)?;

            grid[loc.0][loc.1 + 2] = ']';
            grid[loc.0][loc.1 + 1] = '[';
        
        }
    }

    if d == (-1, 0) {
        if loc.0 < 1 {
            return Err(format!("Cannot move up"));
            
        } else {
            let (leftcol, rightcol) = if grid[loc.0][loc.1] == '[' {
                (loc.1, loc.1 + 1)
            } else {
                (loc.1 - 1, loc.1)
            };

            if grid[loc.0 - 1][leftcol] == '#' || grid[loc.0 - 1][rightcol] == '#' {
                return Err(format!("Wall crash"));

            } else {
                grid = push_box(grid, rows, cols, (loc.0 - 1, leftcol), d)?;
                grid = push_box(grid, rows, cols, (loc.0 - 1, rightcol), d)?;

                grid[loc.0 - 1][leftcol] = '[';
                grid[loc.0 - 1][rightcol] = ']';

                grid[loc.0][leftcol] = '.';
                grid[loc.0][rightcol] = '.';
            
            }
        }
    }

    if d == (1, 0) {
        if loc.0 + 1 >= rows {
            return Err(format!("Cannot move down"));
            
        } else {
            let (leftcol, rightcol) = if grid[loc.0][loc.1] == '[' {
                (loc.1, loc.1 + 1)
            } else {
                (loc.1 - 1, loc.1)
            };

            if grid[loc.0 + 1][leftcol] == '#' || grid[loc.0 + 1][rightcol] == '#' {
                return Err(format!("Wall crash"));

            } else {
                grid = push_box(grid, rows, cols, (loc.0 + 1, leftcol), d)?;
                grid = push_box(grid, rows, cols, (loc.0 + 1, rightcol), d)?;

                grid[loc.0 + 1][leftcol] = '[';
                grid[loc.0 + 1][rightcol] = ']';

                grid[loc.0][leftcol] = '.';
                grid[loc.0][rightcol] = '.';
            
            }
        }
    }

    grid[loc.0][loc.1] = '.';
    Ok(grid)
}