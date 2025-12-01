pub fn day_1_part_2(input: String) {
    let mut dial = 50;
    let mut zero_hits = 0;

    for line in input.lines() {
        let direction = if &line[0..1] == "L" { -1 } else { 1 };
        let mut distance: i32 = line[1..].trim().parse().unwrap();

        while distance > 0 {
            dial += direction;
            distance -= 1;

            if dial < 0 {
                dial += 100;
            } else if dial > 99 {
                dial -= 100;
            }

            if dial == 0 {
                zero_hits += 1;
            }
        }
    }

    println!("");
    println!("###");
    println!("");

    // expected test output: 6
    println!("zero hits: {}", zero_hits);
}