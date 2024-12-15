struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

pub fn run(filename: String, ignore: i32) {
    use regex::Regex;

    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut raw_data_iter = raw_data.lines();
    let width: i32 = raw_data_iter.next().and_then(|x| x.parse().ok()).unwrap();
    let height: i32 = raw_data_iter.next().and_then(|x| x.parse().ok()).unwrap();

    let re = Regex::new("p=([-0-9]+),([-0-9]+) v=([-0-9]+),([-0-9]+)").unwrap();
    let mut details: Vec<Robot> = Vec::new();
    for l in raw_data_iter {
        let (_, [px, py, vx, vy]) = re.captures(l).unwrap().extract();
        let position = (px.parse().unwrap(), py.parse().unwrap());
        let velocity = (vx.parse().unwrap(), vy.parse().unwrap());
        details.push(Robot { position, velocity });
    }

    if ignore != 1 {
        println!("Part 1: {}", get_safety_factor(&details, width, height));
    }

    if ignore != 2 {
        println!("Part 2: {}", look_for_easter_egg(details, width, height));
    }
}

fn get_safety_factor(data: &Vec<Robot>, width: i32, height: i32) -> i32 {
    let mut counts = [0; 4];
    let mut ignored = 0;
    for r in data {
        let x = ((r.position.0 + r.velocity.0 * 100) % width + width) % width;
        let y = ((r.position.1 + r.velocity.1 * 100) % height + height) % height;

        // println!("{x} {y} {width} {height}");

        if x < width / 2 {
            if y < height / 2 {
                counts[0] += 1;
            } else if y > height / 2{
                counts[1] += 1;
            } else {
                ignored += 1;
            }
        } else if x > width / 2 {
            if y < height / 2 {
                counts[2] += 1;
            } else if y > height / 2 {
                counts[3] += 1;
            } else {
                ignored += 1;
            }
        } else {
            ignored += 1;
        }
    }

    println!("{:?} {ignored}", counts);
    counts.into_iter().product()
}

fn look_for_easter_egg(mut data: Vec<Robot>, width: i32, height: i32) -> i32 {
    for i in 1..10404 {
        let mut seen: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];

        for r in data.iter_mut() {
            r.position.0 = (r.position.0 + r.velocity.0 + width) % width;
            r.position.1 = (r.position.1 + r.velocity.1 + height) % height;
            let (x, y): (usize, usize) = (r.position.1.try_into().unwrap(), r.position.0.try_into().unwrap());
            seen[x][y] = '1';
        }

        for row in seen.iter().map(|x| (*x).iter().collect::<String>()) {
            if row.contains("11111111111111") {
                println!("Output at time t = {i}");
                for row in seen {
                    for c in row {
                        print!("{}", char::from(c));
                    }
                    println!("");
                }
                break;
            }
        }
    }

    return 0;
}