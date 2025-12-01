mod days;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: aoc2025 <day> <1 / 2> <optional: test>");
    }

    let day: u32 = args[1].parse().expect("day must be a number");
    let part: u32 = args[2].parse().expect("part must be a number");

    let use_test_input = if args.len() == 4 {
        &args[3] == "test"
    } else {
        false
    };

    let input = get_input(day, use_test_input);
    let solution_fn = get_solution_fn(day, part);
    
    solution_fn(input);
}

fn get_input(day: u32, use_test_input: bool) -> String {
    let input_file = if use_test_input { "test" } else { "input" };
    let input_path = format!("input/{}/{}.txt", day, input_file);

    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));

    return input;
}

fn get_solution_fn(day: u32, part: u32) -> fn(String) {
    match (day, part) {
        (1, 1) => days::day01::part1::day_1_part_1,
        (1, 2) => days::day01::part2::day_1_part_2,
        _ => panic!("Day {} Part {} is not implemented yet", day, part),
    }
}