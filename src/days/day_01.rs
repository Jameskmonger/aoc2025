pub fn day_01(input: String, part: u8) {
    if part == 1 {
        day_1_part_1(input);
    } else if part == 2 {
        day_1_part_2(input);
    } else {
        panic!("Unrecognised part {}", part);
    }
}

fn day_1_part_1(input: String) {
    let mut dial = 50;
    let mut zero_hits = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let distance: i32 = line[1..].trim().parse().unwrap();

        if direction == "L" {            
            dial -= distance;
        } else if direction == "R" {
            dial += distance;
        } else {
            panic!("unknown direction");
        }

        while dial < 0 {
            dial += 100;
        }

        while dial > 99 {
            dial -= 100;
        }

        if dial == 0 {
            zero_hits += 1;
        }

        println!("{}{} to {}", direction, distance, dial);
    }

    println!("");
    println!("###");
    println!("");

    // expected test output: 3
    println!("zero hits: {}", zero_hits);
}

fn day_1_part_2(input: String) {
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