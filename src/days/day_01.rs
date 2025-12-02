pub fn day_01(input: &str, part: u8) {
    let mut dial = 50;
    let mut zero_hits = 0;

    for line in input.lines() {
        let (direction_str, distance_str) = line.split_at(1);
        let direction: i8 = match direction_str {
            "L" => -1,
            "R" => 1,
            other => panic!("unsupported direction: {}", other),
        };

        let mut distance: u32 = distance_str.trim().parse().expect("unable to parse distance");

        while distance > 0 {
            dial += direction;
            distance -= 1;

            if dial < 0 {
                dial += 100;
            } else if dial > 99 {
                dial -= 100;
            }

            // part 2 solution increments counter for any time it passes 0
            if part == 2 && dial == 0 {
                zero_hits += 1;
            }
        }

        // part 1 solution only increments the counter if it lands on 0
        if part == 1 && dial == 0 {
            zero_hits += 1;
        }
    }

    println!("zero hits: {}", zero_hits);
}
