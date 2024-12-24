use std::collections::{HashMap, VecDeque};

pub fn run(filename: String, ignore: i32) {
    let raw_data = std::fs::read_to_string(filename.clone()).unwrap();
    let mut lines_iter = raw_data.lines();

    let initials: HashMap<&str, bool> = HashMap::from_iter(
        lines_iter.by_ref()
            .map_while(
                |x| x.split_once(": ")
            )
            .map(
                |(name, bit)| (name, bit == "1")
            )
    );

    let gate_logic: Vec<Vec<&str>> = lines_iter
        .map(
            |l| l.split_ascii_whitespace().collect()
        )
        .collect();

    if ignore != 1 {
        println!("Part 1: {}", run_circuit(&initials, &gate_logic));
    }

    if ignore != 2 && filename.contains("real") {
        println!("Part 2: {}", find_full_adders(&gate_logic));
    }
}

struct Gate {
    operation: fn(bool, bool) -> bool,

    inputs_received: i32,
    left: bool, right: bool,

    output: bool
}

impl Gate {
    fn build(operation: fn(bool, bool) -> bool) -> Gate {
        Gate {
            operation, 
            inputs_received: 0, 
            left: false, right: false,
            output: false
        }
    }
    
    fn add_input(&mut self, value: bool, on_left: bool) -> Option<bool> {
        if on_left { self.left = value; } else { self.right = value; }
        self.inputs_received += 1;

        if self.inputs_received == 2 {
            self.output = (self.operation)(self.left, self.right);
            return Some(self.output);
        }

        None
    }
}

fn run_circuit(initials: &HashMap<&str, bool>, gate_logic: &Vec<Vec<&str>>) -> i64 {
    let mut gates = Vec::new();
    let mut wire_leads: HashMap<&str, (Vec<usize>, Vec<usize>)> = HashMap::new();
    let mut z_values= Vec::new();

    for x in gate_logic {
        let l = gates.len();

        let operation = match x[1] {
            "AND" => |x, y| x && y,
            "OR" => |x, y| x || y,
            "XOR" => |x, y| (x || y) && (!x || !y),
            _ => panic!("Unexpected Gate Name")
        };
        gates.push((Gate::build(operation), x[4]));
        
        if let Some(g) = wire_leads.get_mut(x[0]) {
            (*g).0.push(l);
        } else {
            wire_leads.insert(x[0], (vec![l], vec![]));
        }

        if let Some(g) = wire_leads.get_mut(x[2]) {
            (*g).1.push(l);
        } else {
            wire_leads.insert(x[2], (vec![], vec![l]));
        }
    }

    let mut queue = VecDeque::new();
    for (name, bit) in initials {
        queue.push_back((*name, *bit));
    }

    while let Some((name, bit)) = queue.pop_front() {
        if let Some((left, right)) = wire_leads.get(name) {
            
            for l in left {
                if let Some(x) = gates[*l].0.add_input(bit, true) {
                    let output_name = gates[*l].1;
                    queue.push_back((output_name, x));

                    if let Some(n) = output_name.strip_prefix('z') {
                        z_values.push((n, x));
                    }
                }
            }

            for r in right {
                if let Some(x) = gates[*r].0.add_input(bit, false) {
                    let output_name = gates[*r].1;
                    queue.push_back((output_name, x));
                    
                    if let Some(n) = output_name.strip_prefix('z') {
                        z_values.push((n, x));
                    }
                }
            }

        }
    }

    z_values.sort();
    z_values.reverse();

    let mut count = 0;
    for (_, b) in z_values {
        count <<= 1;
        if b { count += 1; }
    }

    return count;
}

// The following code tries to check the circuit for full adders (one per pair of xi, yi)
// Any erroneous adder is flagged and printed out
// The first errorenous adder is checked and the required swap is manually noted down in the helper24.txt file
// The code is run again with the swap in place, and the process if repeated until the adders are perfect
fn find_full_adders(gate_logic: &Vec<Vec<&str>>) -> String {
    let mut skips: HashMap<&str, &str> = HashMap::new();
    let skip_values = std::fs::read_to_string("./data/helper24.txt").unwrap();
    for (a, b) in skip_values.lines().map_while(|l| l.split_once(' ')) {
        skips.insert(a, b);
    }

    let mut gate_details = Vec::new();
    let mut wire_leads: HashMap<&str, Vec<usize>> = HashMap::new();

    for x in gate_logic {
        let l = gate_details.len();
        let mut temp = vec![x[0], x[2]];
        temp.sort();

        if let Some(n) = skips.get(x[4]) {
            gate_details.push((x[1], temp.clone(), *n));
        } else {
            gate_details.push((x[1], temp.clone(), x[4]));
        }
        
        if let Some(g) = wire_leads.get_mut(temp[0]) {
            (*g).push(l);
        } else {
            wire_leads.insert(temp[0], vec![l]);
        }

        if let Some(g) = wire_leads.get_mut(temp[1]) {
            (*g).push(l);
        } else {
            wire_leads.insert(temp[1], vec![l]);
        }
    }

    let mut prev_carry = "nvv";
    for i in 1..45 {
        let mut num = i.to_string();
        if i < 10 {
            num.insert(0, '0');
        }

        let mut xname = num.clone();
        xname.insert(0, 'x');

        let mut yname = num.clone();
        yname.insert(0, 'y');

        let mut zname = num;
        zname.insert(0, 'z');

        if let Some(g) = wire_leads.get(&&xname[..]) {
            let mut temp: Vec<_> = g.into_iter().map(|l| gate_details[*l].clone()).collect();
            temp.sort();

            let mut check = || -> Result<(), String> {
                let mut intermediates = Vec::new();

                match &temp[0] {
                    ("AND", inputs, output) if inputs[0] == xname && inputs[1] == yname => {
                        intermediates.push(output);
                        Ok(())
                    }

                    _ => Err("Invalid1")
                }?;

                match &temp[1] {
                    ("XOR", inputs, output) if inputs[0] == xname && inputs[1] == yname => {
                        intermediates.push(output);
                        Ok(())
                    }

                    _ => Err("Invalid2")
                }?;

                let mut temp2: Vec<_> = wire_leads.get(intermediates[1]).ok_or("Invalid3")?
                    .into_iter().map(|l| gate_details[*l].clone()).collect();
                temp2.sort();

                match &temp2[0] {
                    ("AND", inputs, output) 
                    if inputs.contains(&prev_carry) => {
                        intermediates.push(output);
                        Ok(())
                    }

                    _ => Err("Invalid4")
                }?;

                match &temp2[1] {
                    ("XOR", inputs, output) 
                    if *output == &zname[..] && inputs.contains(&prev_carry) => {
                        Ok(())
                    }

                    _ => Err("Invalid5")
                }?;

                let carry_gate = wire_leads.get(intermediates[2]).ok_or("Invalid6")?;
                if carry_gate.len() != 1 {
                    return Err(String::from("Invalid7"));
                }

                match &gate_details[carry_gate[0]] {
                    ("OR", inputs, output) 
                    if inputs.contains(intermediates[0]) => {
                        prev_carry = output;
                        Ok(())
                    }

                    _ => Err("Invalid8")
                }?;

                Ok(())
            };

            if let Err(e) = check() { 
                println!("{e}");
                for detail in temp {
                    println!("{:?}", detail);

                    if let Some(g1) = wire_leads.get(&detail.2) {
                        print!("->");
                        for l in g1 {
                            print!(" {:?}", gate_details[*l]);
                        }
                        print!("\n");
                    }
                }

                println!("\n");
            }
        }
    }
    
    let mut swapped: Vec<&str> = skips.into_keys().collect();
    swapped.sort();
    return swapped.join(",");
}