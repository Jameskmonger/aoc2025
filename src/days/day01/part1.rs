pub fn day_1_part_1(input: String) {
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