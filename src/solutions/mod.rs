mod historian_hysteria;
mod red_nosed_reports; mod mull_it_over;
mod ceres_search; mod print_queue;
mod guard_gallivant; mod bridge_repair;
mod resonant_collinearity; mod disk_fragmenter;
mod hoof_it; mod plutonian_pebbles; mod garden_groups;
mod claw_contraption; mod restroom_redoubt;
mod warehouse_woes; mod reindeer_maze;
mod chronospatial_computer; mod ram_run;
mod linen_layout; mod race_condition;
mod keypad_conundrum; mod monkey_market; mod lan_party;
mod crossed_wires; mod code_chronicle;

pub fn run(day: i32, mode: String, ignore: i32) {
    let filename = format!("./data/day{day}_{mode}.txt");
    match day {
        1 => historian_hysteria::run(filename, ignore),
        2 => red_nosed_reports::run(filename, ignore),
        3 => mull_it_over::run(filename, ignore),
        4 => ceres_search::run(filename, ignore),
        5 => print_queue::run(filename, ignore),
        6 => guard_gallivant::run(filename, ignore),
        7 => bridge_repair::run(filename, ignore),
        8 => resonant_collinearity::run(filename, ignore),
        9 => disk_fragmenter::run(filename, ignore),
        10 => hoof_it::run(filename, ignore),
        11 => plutonian_pebbles::run(filename, ignore),
        12 => garden_groups::run(filename, ignore),
        13 => claw_contraption::run(filename, ignore),
        14 => restroom_redoubt::run(filename, ignore),
        15 => warehouse_woes::run(filename, ignore),
        16 => reindeer_maze::run(filename, ignore),
        17 => chronospatial_computer::run(filename, ignore),
        18 => ram_run::run(filename, ignore),
        19 => linen_layout::run(filename, ignore),
        20 => race_condition::run(filename, ignore),
        21 => keypad_conundrum::run(filename, ignore),
        22 => monkey_market::run(filename, ignore),
        23 => lan_party::run(filename, ignore),
        24 => crossed_wires::run(filename, ignore),
        25 => code_chronicle::run(filename, ignore),
        _ => panic!("No solution implemented for day {day}.")
    };
}