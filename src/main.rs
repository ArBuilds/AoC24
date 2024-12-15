use std::env;
mod solutions;

fn main() {
    let mut user_args = env::args();

    let day = user_args.nth(1).expect("Insufficient arguments. Program requires argument to specify day number.")
        .parse::<i32>().expect("Invalid argument. Expected day number, found not integer.");
    let mode = user_args.next().expect("Insufficient arguments. Program requires argument to specify run mode.");

    let mut ignore = 0;
    if let Some(ignore_part) = user_args.next() {
        ignore = ignore_part.parse::<i32>().expect("Can only ignore 1 or 2");
    }
    
    println!("Running day {day} in {mode} mode!");
    solutions::run(day, mode, ignore);
}