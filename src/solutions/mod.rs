mod historian_hysteria;
mod red_nosed_reports; mod mull_it_over;
mod ceres_search; mod print_queue;
mod guard_gallivant; mod bridge_repair;
mod resonant_collinearity; mod disk_fragmenter;
mod hoof_it; mod plutonian_pebbles; mod garden_groups;
mod claw_contraption; mod restroom_redoubt;
mod warehouse_woes;

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
        _ => panic!("No solution implemented for day {day}.")
    };
}