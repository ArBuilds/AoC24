pub fn run(filename: String, ignore: i32) {
    let data = std::fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", checksum_after_compacted(&data));
    }

    if ignore != 2 {
        println!("Part 2: {}", checksum_after_compacted_whole(&data));
    }
}

fn checksum_after_compacted(data: &String) -> i64 {
    use std::collections::VecDeque;

    let mut checksum: i64 = 0;
    let mut values: VecDeque<i64> = data.chars()
        .map(|x| x.to_digit(10).and_then(|x| TryInto::<i64>::try_into(x).ok()).unwrap())
        .collect();

    let mut curr_id = 0; let mut last_id: i64 = (values.len() / 2).try_into().unwrap();
    let mut loc = 0; let mut last_sum = 0;
    
    for i in 0.. {
        if i % 2 == 1 { // if we're dealing with a gap
            while values.len() > 0 && values[0] >= values[values.len() - 1] {
                if let Some(filelen) = values.pop_back() {
                    let temp = (loc + filelen - 1) * (loc + filelen) / 2;
                    checksum += (temp - last_sum) * last_id;
                    last_id -= 1;
                    last_sum = temp;
                    loc += filelen;
                    values[0] -= filelen;

                } else {
                    return checksum;
                }

                values.pop_back(); // remove gap at end
            }
        }

        if let Some(filelen) = values.pop_front() {
            let temp = (loc + filelen - 1) * (loc + filelen) / 2;
            if i % 2 == 0 {
                checksum += (temp - last_sum) * curr_id;
                curr_id += 1;
            } else {
                checksum += (temp - last_sum) * last_id;
                let l = values.len();
                values[l - 1] -= filelen;
                if values[l - 1] == 0 {
                    last_id -= 1;
                }
            }

            last_sum = temp;
            loc += filelen;

        } else {
            return checksum;
        }
    }

    return checksum;
}

fn checksum_after_compacted_whole(data: &String) -> i64 {
    use std::collections::VecDeque;

    let mut checksum: i64 = 0;
    let mut files = VecDeque::new();
    let mut gaps = VecDeque::new();

    let mut curr_loc = 0;
    for (i, n) in data.chars()
        .map(|x| x.to_digit(10).and_then(|x| TryInto::<i64>::try_into(x).ok()).unwrap())
        .enumerate() 
    {
        if i % 2 == 0 {
            files.push_back((n, curr_loc));
        } else {
            gaps.push_back((n, curr_loc));
        }
        
        curr_loc += n;
    }

    let l = files.len();
    for (f, id) in files.into_iter().zip(0..l).rev() {
        let (size, mut loc) = f;
        for g in gaps.iter_mut() {
            if (*g).0 >= size {
                (*g).0 -= size;
                loc = (*g).1;
                (*g).1 += size;
                break;
            }
        }
        
        checksum += ((loc + size - 1) * (loc + size) - (loc - 1) * (loc)) / 2 * TryInto::<i64>::try_into(id).unwrap();
        gaps.pop_back();
    }

    return checksum;
}