pub fn run(filename: String, ignore: i32) {
    let data = std::fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", 
            data.lines().filter(|l| is_safe(l)).count()
        );
    }

    if ignore != 2 {
        println!("Part 2: {}", 
            data.lines().filter(|l| is_safe_dampened(l)).count()
        );
    }
}

fn is_safe(report: &str) -> bool {
    aux(report) == -1
}

fn aux(report: &str) -> i32 {
    let mut numbers = report.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let mut prev = numbers.nth(0).unwrap();
    let mut current = numbers.nth(0).unwrap();

    let is_increasing = current > prev;
    let mut i = 1;
    loop {
        let big = if is_increasing { current } else { prev };
        let small = current + prev - big;

        if big - small < 1 || big - small > 3 {
            return i;
        }

        if let Some(l) = numbers.next() {
            prev = current; current = l;
            i += 1;
        } else {
            break;
        }
    }

    -1
}

fn is_safe_dampened(report: &str) -> bool {
    let temp = aux(report);
    
    if temp == -1 {
        return true;
    }

    let mut values: Vec<&str> = report.split_whitespace().collect();
    let values_temp = values.clone(); 
    
    values.remove(temp.try_into().unwrap());
    if aux(&values.join(" ")[..]) == -1 {
        return true;
    }

    values = values_temp.clone();
    values.remove((temp - 1).try_into().unwrap());
    if aux(&values.join(" ")[..]) == -1 {
        return true;
    }

    values = values_temp.clone();
    values.remove(0);
    if aux(&values.join(" ")[..]) == -1 {
        return true;
    }

    false
}