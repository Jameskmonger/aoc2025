mod days;

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: aoc2025 <day> <1 / 2> <optional: test>");
    }

    let day: u8 = args[1].parse().expect("day must be a number");
    let part: u8 = args[2].parse().expect("part must be a number");

    let use_test_input = args.len() == 4 && args[3] == "test";

    let input = get_input(day, use_test_input);
    let solution_fn = get_solution_fn(day);
    
    let start = Instant::now();
    solution_fn(&input, part);
    let elapsed = start.elapsed();

    println!();

    println!(
        "executed day {} part {} (with {} input) in {} μs",
        day,
        part,
        if use_test_input { "test" } else { "real" },
        elapsed.as_micros());
}

fn get_input(day: u8, use_test_input: bool) -> String {
    let input_file = if use_test_input { "test" } else { "input" };
    let input_path = format!("input/{}/{}.txt", day, input_file);

    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));

    return input;
}

fn get_solution_fn(day: u8) -> fn(&str, u8) {
    match day {
        1 => days::day_01::day_01,
        2 => days::day_02::day_02,
        _ => panic!("Day {} is not implemented yet", day),
    }
}