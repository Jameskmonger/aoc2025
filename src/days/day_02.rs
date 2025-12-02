pub fn day_02(input: &str, part: u8) {
    let repeat_check_fn = if part == 1 {
        is_repeated_pattern
    } else {
        contains_repeated_pattern
    };

    let ranges = input.split(',');

    let mut sum = 0;

    for range in ranges {
        let mut bounds = range
            .split('-')
            .map(|x| x.parse::<u64>().expect("could not parse range part"));

        let start = bounds.next().expect("could not get start from range");
        let end = bounds.next().expect("could not get end from range");

        let repeated = (start..=end)
            .filter(|n| repeat_check_fn(n));

        sum += repeated.sum::<u64>();
    }

    println!("sum of invalid ids: {}", sum);
}

fn is_repeated_pattern(n: &u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // length must be even for the string to consist solely of a repeated pattern
    if len % 2 != 0 {
        return false;
    }

    let (part_a, part_b) = s.split_at(len / 2);

    return part_a == part_b;
}

fn contains_repeated_pattern(n: &u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    let midpoint = len / 2;

    return (1..=midpoint)
        .filter(|l| len % l == 0)
        .any(|pattern_len| {
            // split the string into chunks of the pattern length
            let mut chunks = s
                .as_bytes()
                .chunks(pattern_len);

            // all chunks are equal to the 1st then the pattern repeats
            let first = chunks.next().unwrap();
            return chunks.all(|c| c == first);
        });
}
