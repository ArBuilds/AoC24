use itertools::Itertools;

pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename.clone()).unwrap();
    let mut iter = raw_data.lines();

    let registers: Vec<i64> = iter.by_ref().take(3)
        .map(|x| x.split_at(12).1.parse().unwrap())
        .collect();
    let code = iter.nth(1).expect("Program starts after a 1 line gap")
        .split_at(9).1
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", execute_op(&code, registers.clone()));
    }

    if ignore != 2 {
        if filename.contains("real") {
            println!("Part 2: {}", only_for_puzzle_input(&code));
        } else {
            println!("Part 2: {}", debug_register_a(&code, registers));
        }
    }
}

fn execute_op(code: &Vec<i64>, mut registers: Vec<i64>) -> String {
    let mut outputs: Vec<i64> = Vec::new();
    let combo = |n: i64, r: &Vec<i64>| {
        if 0 > n || n >= 7 {
            panic!("Invalid combo operand {n}");
        } else if n <= 3 {
            n
        } else if n == 4 {
            r[0]
        } else if n == 5 {
            r[1]
        } else {
            r[2]
        }
    };

    let mut pointer = 0;
    while pointer < code.len() {
        match code[pointer] {
            0 => registers[0] >>= combo(code[pointer + 1], &registers),
            1 => registers[1] ^= code[pointer + 1],
            2 => registers[1] = combo(code[pointer + 1], &registers) % 8,
            3 => { if registers[0] != 0 { pointer = code[pointer + 1].try_into().unwrap(); continue; }}
            4 => registers[1] ^= registers[2],
            5 => { outputs.push(combo(code[pointer + 1], &registers) % 8); }
            6 => registers[1] = registers[0] >> combo(code[pointer + 1], &registers),
            7 => registers[2] = registers[0] >> combo(code[pointer + 1], &registers),
            _ => panic!("Invalid opcode {}", code[pointer])
        };

        // println!("{:?} {} {}", registers, code[pointer], code[pointer + 1]);
        pointer += 2;
    }

    return outputs.into_iter()
        .map(|x| x.to_string()).join(",");
}

fn debug_register_a(code: &Vec<i64>, registers_init: Vec<i64>) -> i64 {
    let combo = |n: i64, r: &Vec<i64>| {
        if 0 > n || n >= 7 {
            panic!("Invalid combo operand {n}");
        } else if n <= 3 {
            n
        } else if n == 4 {
            r[0]
        } else if n == 5 {
            r[1]
        } else {
            r[2]
        }
    };

    let upper_lim = 1_000_000;
    for i in 1..upper_lim {
        let mut registers = registers_init.clone();
        registers[0] = i;

        let mut output_len = 0;
        let mut pointer = 0;
        while pointer < code.len() {
            match code[pointer] {
                0 => registers[0] >>= combo(code[pointer + 1], &registers),
                1 => registers[1] ^= code[pointer + 1],
                2 => registers[1] = combo(code[pointer + 1], &registers) % 8,
                3 => { if registers[0] != 0 { pointer = code[pointer + 1].try_into().unwrap(); continue; }}
                4 => registers[1] ^= registers[2],
                5 => { 
                    if combo(code[pointer + 1], &registers) % 8 != code[output_len] {
                        break;
                    } else {
                        output_len += 1;
                    }
                }
                6 => registers[1] = registers[0] >> combo(code[pointer + 1], &registers),
                7 => registers[2] = registers[0] >> combo(code[pointer + 1], &registers),
                _ => panic!("Invalid opcode {}", code[pointer])
            };

            pointer += 2;
        }

        if output_len == code.len() {
            return i;
        }
    }

    return -1;
}

fn only_for_puzzle_input(code: &Vec<i64>) -> i64 {
    /*
        Working:
        On inspecting the puzzle input:
            if l = last 3 bits of A
            output is an xor of l, (last 3 bits of A (excluding l bits)), and other values
            then remove the last 3 bits of A and repeat process

        Solution:
            Create a vec containing all valid bit combinations (-1 if the bit can hold either 0 or 1)

            Iterate through every output (looking at the 3 bits starting from i * 3)
            Create new_options (bit combinations where the current 3 bits are set to all possible values)
            
            For each of these options to be valid, 
            three of the bits must be set to output ^ l ^ (other predefined values)
            If any of the three bits are already set to a different value, exclude the combination, else continue            
    */

    let mut valid_bit_options: Vec<Vec<i64>> = vec![vec![-1; code.len() * 3 + 8]];
    
    fn new_option_generator(op_list: &mut Vec<Vec<i64>>, op: &mut Vec<i64>, pos: usize, len: i64) {
        if len == 0 {
            op_list.push(op.to_vec());
            return;
        }

        if op[pos] == -1 {
            op[pos] = 0;
            new_option_generator(op_list, op, pos + 1, len - 1);
            op[pos] = 1;
            new_option_generator(op_list, op, pos + 1, len - 1);
            op[pos] = -1;
        } else {
            new_option_generator(op_list, op, pos + 1, len - 1);
        }
    }

    for i in 0..code.len() {
        let mut new_options: Vec<Vec<i64>> = Vec::new();
        for b in valid_bit_options.iter_mut() {
            new_option_generator(&mut new_options, b, i * 3, 3);
        }

        valid_bit_options.clear();
        for mut b in new_options {
            let last3 = b[i * 3] + 2 * b[i * 3 + 1] + 4 * b[i * 3 + 2];
            
            let mut to_set = code[i] ^ 3 ^ last3;
            let pos: usize = (last3 ^ 5).try_into().unwrap();
            let mut at = 0;
            while at < 3 {
                if b[i * 3 + pos + at] != -1 && b[i * 3 + pos + at] != to_set % 2 {
                    break;
                }

                b[i * 3 + pos + at] = to_set % 2;
                to_set >>= 1;
                at += 1;
            }

            if at == 3 {
                valid_bit_options.push(b);
            }
        }
    }

    let mut v: Vec<i64> = valid_bit_options.into_iter().map(
            |b| {
                let mut value: i64 = 0;
                let mut power = 1;
                for x in b.iter() {
                    if *x == 1 {
                        value += power;
                    }

                    power <<= 1;
                }
                value
            }
        )
        .collect();

    v.sort();
    println!("{}", execute_op(code, vec![v[0], 0, 0]));
    return v[0];
}