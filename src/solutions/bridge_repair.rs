pub fn run(filename: String, ignore: i32) {
    let data = std::fs::read_to_string(filename).unwrap();

    if ignore != 1 {
        println!("Part 1: {}", 
            data.lines()
                .filter_map(|l| {
                    let (t, values) = l.split_once(':').unwrap();
                    let target = t.parse::<i64>().unwrap();
                    let mut rev_operands: Vec<i64> = values.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
                    rev_operands.reverse();
                    if is_produceable(target, &mut rev_operands) {
                        Some(target)
                    } else {
                        None
                    }
                })
                .sum::<i64>()
        );
    }

    if ignore != 2 {
        println!("Part 2: {}",
            data.lines()
                .filter_map(|l| {
                    let (t, values) = l.split_once(':').unwrap();
                    let target = t.parse::<i64>().unwrap();
                    let mut rev_operands: Vec<i64> = values.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
                    rev_operands.reverse();
                    if is_produceable_with_conc(target, &mut rev_operands) {
                        Some(target)
                    } else {
                        None
                    }
                })
                .sum::<i64>()
        );
    }
}

fn is_produceable(target: i64, rev_operands: &mut Vec<i64>) -> bool {
    if rev_operands.len() == 0 {
        false

    } else if rev_operands.len() == 1 {
        target == rev_operands[0]

    } else {
        let l = rev_operands.len();
        let last = rev_operands[l - 1]; rev_operands.pop();

        if target >= rev_operands[l - 2] + last {
            rev_operands[l - 2] += last;
            if is_produceable(target, rev_operands) {
                return true;
            } else {
                rev_operands[l - 2] -= last;
            }
        }

        if target >= rev_operands[l - 2] * last {
            rev_operands[l - 2] *= last;
            if is_produceable(target, rev_operands) {
                return true;
            } else {
                rev_operands[l - 2] /= last;
            }
        }

        rev_operands.push(last);
        return false;
    }
}

fn is_produceable_with_conc(target: i64, rev_operands: &mut Vec<i64>) -> bool {
    if rev_operands.len() == 0 {
        false

    } else if rev_operands.len() == 1 {
        target == rev_operands[0]

    } else {
        let l = rev_operands.len();
        let last = rev_operands[l - 1]; rev_operands.pop();

        if target >= rev_operands[l - 2] + last {
            rev_operands[l - 2] += last;
            if is_produceable_with_conc(target, rev_operands) {
                return true;
            } else {
                rev_operands[l - 2] -= last;
            }
        }

        if target >= rev_operands[l - 2] * last {
            rev_operands[l - 2] *= last;
            if is_produceable_with_conc(target, rev_operands) {
                return true;
            } else {
                rev_operands[l - 2] /= last;
            }
        }

        let non_conc_ed = rev_operands[l - 2];
        let conc_ed = conc(last, rev_operands[l - 2]);
        if target >= conc_ed {
            rev_operands[l - 2] = conc_ed;
            if is_produceable_with_conc(target, rev_operands) {
                return true;
            } else {
                rev_operands[l - 2] = non_conc_ed;
            }
        }

        rev_operands.push(last);
        return false;
    }
}

fn conc(a: i64, b: i64) -> i64 {
    // only +ve numbers
    let mut power = 1;
    while b / power > 0 {
        power *= 10;
    }

    a * power + b
}