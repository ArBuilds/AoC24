use std::collections::HashMap;
use itertools::Itertools;

pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in raw_data.lines() {
        let a = &l[..2]; let b = &l[3..];

        if let Some(x) = connections.get_mut(&a) {
            (*x).push(b);
        } else {
            connections.insert(a, vec![b]);
        }

        if let Some(x) = connections.get_mut(&b) {
            (*x).push(a);
        } else {
            connections.insert(b, vec![a]);
        }
    }

    if ignore != 1 {
        println!("Part 1: {}", find_triplets_with_t(&connections));
    }

    if ignore != 2 {
        println!("Part 2: {}", find_largest_full_mesh(&connections));
    }
}

fn find_triplets_with_t(connections: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut count = 0;
    let mut seen = Vec::new();

    for (k, v) in connections {
        if !k.starts_with("t") {
            continue;
        }

        seen.push(*k);
        for pair in v.iter().combinations(2) {
            if seen.contains(pair[0]) || seen.contains(pair[1]) {
                continue;
            }

            let x = connections.get(pair[0]).expect("Computer is connected");
            if x.contains(pair[1]) {
                count += 1;
            }
        }
    }
    
    return count;
}

fn find_largest_full_mesh(connections: &HashMap<&str, Vec<&str>>) -> String {
    // We know that several triplets exist, so the largest mesh has to be a quadruplet or bigger

    let mut seen = Vec::new();
    let mut triplets = Vec::new();

    for (k, v) in connections {
        seen.push(*k);

        for pair in v.iter().combinations(2) {
            if seen.contains(pair[0]) || seen.contains(pair[1]) {
                continue;
            }

            let x = connections.get(pair[0]).expect("Computer is connected");
            if x.contains(pair[1]) {
                let mut temp = vec![*k, *pair[0], *pair[1]];
                temp.sort();
                triplets.push(temp);
            }
        }
    }

    let mut largest_size = 3;
    let mut bigger_meshes = Vec::new();

    for mut mesh in triplets {
        let start = mesh[0];
        for computer in connections.get(start).expect("Computer is connected") {

            if mesh.iter()
                .all(
                    |x| connections.get(x).expect("Computer is connected").contains(computer)
                )
            {
                if mesh.len() == largest_size {
                    largest_size += 1;
                }

                mesh.push(*computer);
            }

        }

        bigger_meshes.push(mesh);
    }

    let mut result = bigger_meshes.into_iter().find(|x| x.len() == largest_size).unwrap();
    // bigger_meshes also contains permutations of the same mesh, so find the first largest size mesh
    
    result.sort();
    return result.join(",");
}