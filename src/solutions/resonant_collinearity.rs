pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let data: Vec<Vec<char>> = raw_data.lines().map(|l| l.chars().collect()).collect();
    let rows = data.len(); let cols = data[0].len();

    if ignore != 1 {
        println!("Part 1: {}", compute_antinodes(&data, rows, cols));
    }

    if ignore != 2 {
        println!("Part 2: {}", compute_antinodes_with_resonance(&data, rows, cols));
    }
}

fn compute_antinodes(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    use std::collections::HashMap; use itertools::Itertools;

    let mut count = 0;
    let mut seen: Vec<Vec<bool>> = (0..rows).map(|_| (0..cols).map(|_| false).collect()).collect();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c != '.' {
                if let Some(v) = nodes.get_mut(c) {
                    (*v).push((i, j));
                } else {
                    nodes.insert(*c, vec![(i, j)]);
                }
            }
        }
    }

    for node_collection in nodes.into_values() {
        for pair in node_collection.into_iter().permutations(2) {
            let (x1, y1) = pair[0]; let (x2, y2) = pair[1];

            if 2 * x2 < x1 || 2 * y2 < y1{
                continue;
            }

            let (x, y) = (2 * x2 - x1, 2 * y2 - y1);
            if x >= rows || y >= cols {
                continue;
            }

            if !seen[x][y] {
                seen[x][y] = true;
                count += 1;
            }
        }
    }

    return count;
}

fn compute_antinodes_with_resonance(data: &Vec<Vec<char>>, rows: usize, cols: usize) -> i32 {
    use std::collections::HashMap; use itertools::Itertools;

    let mut count = 0;
    let mut seen: Vec<Vec<bool>> = (0..rows).map(|_| (0..cols).map(|_| false).collect()).collect();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, r) in data.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c != '.' {
                if let Some(v) = nodes.get_mut(c) {
                    (*v).push((i, j));
                } else {
                    nodes.insert(*c, vec![(i, j)]);
                }
            }
        }
    }

    for node_collection in nodes.into_values() {
        for pair in node_collection.into_iter().permutations(2) {
            let (x1, y1) = pair[0]; let (x2, y2) = pair[1];
            
            for i in 0usize.. {
                if (i + 1) * x2 < i * x1 || (i + 1) * y2 < i * y1{
                    break;
                }

                let (x, y) = ((i + 1) * x2 - i * x1, (i + 1) * y2 - i * y1);
                if x >= rows || y >= cols {
                    break;
                }

                if !seen[x][y] {
                    seen[x][y] = true;
                    count += 1;
                }
            }
        }
    }

    return count;
}