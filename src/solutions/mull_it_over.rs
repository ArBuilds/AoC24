pub fn run(filename: String, ignore: i32) {
    let data = std::fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", run_mul_commands(&data[..]));
    }

    if ignore != 2 {
        println!("Part 2: {}", run_enabled_mul_commands(&data[..]));
    }
}

fn run_mul_commands(data: &str) -> i32 {
    use regex::Regex;

    let re = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    re.captures_iter(data).map(|c| c.extract().1)
        .map(|[a, b]| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()).sum()
}

fn run_enabled_mul_commands(data: &str) -> i32 {
    use regex::Regex;

    let re = Regex::new(r"mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    let do_dont = Regex::new(r"(.*?)(do(?:n't)?)\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    for (_, [a, b]) in do_dont.captures_iter(&(data.to_owned() + "do()")[..]).map(|c| c.extract()) {
        if enabled {
            sum += re.captures_iter(a).map(|c| c.extract().1)
                .map(|[a, b]| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()).sum::<i32>();
        }

        enabled = b == "do";
    }

    sum
}