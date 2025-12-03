pub fn day_03(input: &str, part: u8) {
    let battery_count = if part == 1 { 2 } else { 12 };

    let joltage_total = input.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .map(|digits| max_joltage(&digits, battery_count))
        .fold(0, |acc, cur| acc + cur);

    println!("joltage total: {}", joltage_total);
}

fn max_joltage(batteries: &[u8], battery_count: usize) -> u64 {
    // batteries must be used in order,
    // so if we need to use all available batteries then we can simply parse the input
    if battery_count == batteries.len() {
        return digits_to_number(batteries);
    }

    // if we can only use 1 battery then we can take the highest individual battery
    if battery_count == 1 {
        return *batteries.iter().max().expect("could not get max value from battery array") as u64;
    }

    // find the highest digit in the valid search range
    // need to leave enough digits after the search range to satisfy the remaining battery count
    let search_end = batteries.len() - battery_count + 1;
    let (max, max_index) = get_highest_individual_joltage(&batteries[..search_end]);

    // recurse through the remaining batteries (after our current max)
    let remaining_batteries = &batteries[max_index + 1..];
    let remaining_joltage = max_joltage(remaining_batteries, battery_count - 1);

    // shift the current result left by the number of remaining batteries and combine them
    let shift = 10u64.pow((battery_count - 1) as u32);
    (max as u64 * shift) + remaining_joltage
}

fn get_highest_individual_joltage(batteries: &[u8]) -> (u8, usize) {
    let mut max = 0u8;
    let mut max_index = usize::MIN;

    for n in 0..batteries.len() {
        let val = batteries[n];

        if val > max {
            max = val;
            max_index = n;
        }
    }

    return (max, max_index);
}

fn digits_to_number(digits: &[u8]) -> u64 {
    digits.iter()
        // shift the acc left by 1 and add the new digit on the end
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}
