use std::ops::RangeInclusive;

pub fn day_05(input: &str, part: u8) {
    let (ranges, ingredients) = parse_input(input);

    if part == 1 {
        let fresh = ingredients_in_range(&ranges, &ingredients)
            .count();
    
        println!("fresh ingredients: {}", fresh);
    } else {
        let fresh = distinct_covered_count(&ranges);

        println!("fresh ingredients: {}", fresh);
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    let mut is_taking_ranges = true;
    for line in input.lines() {
        if line.len() == 0 {
            is_taking_ranges = false;
            continue;
        }

        if is_taking_ranges {
            let parts: Vec<u64> = line
                .split('-')
                .map(|x| x.parse::<u64>().expect("could not parse range part"))
                .collect();

            ranges.push(parts[0]..=parts[1]);
        } else {
            let value = line.parse::<u64>().expect("could not parse ingredient");

            ingredients.push(value);
        }
    }

    (ranges, ingredients)
}

fn ingredients_in_range<'a>(
    ranges: &'a Vec<RangeInclusive<u64>>,
    ingredients: &'a Vec<u64>
) -> impl Iterator<Item = &'a u64> {
    ingredients
        .iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
}

fn distinct_covered_count(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    if ranges.len() == 0 {
        return 0;
    }

    let mut ranges = ranges.clone();

    // sort ranges by start, then end so we can merge them in one pass
    ranges.sort_unstable_by(|a, b| {
        a.start().cmp(b.start())
        .then(a.end().cmp(b.end()))
    });

    let mut total = 0u64;

    let mut cur_start = ranges[0].start();
    let mut cur_end = ranges[0].end();

    for r in &ranges[1..] {
        let start = r.start();
        let end = r.end();

        // if the current range overlaps or touches the previous range, merge them
        if start <= cur_end {
            if end > cur_end {
                cur_end = end;
            }

            continue;
        }

        // add previous range to total and start a new range
        total += cur_end - cur_start + 1;
        cur_start = start;
        cur_end = end;
    }

    // add the last range to total
    total += cur_end - cur_start + 1;

    total
}


#[cfg(test)]
mod test {
    use super::{
        parse_input,
        ingredients_in_range,
        distinct_covered_count
    };

    #[test]
    fn parses_input_correctly() {
        let input = "1-3
5-7
10-12

4
5
6";

        let (ranges, ingredients) = parse_input(input);

        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], 1..=3);
        assert_eq!(ranges[1], 5..=7);
        assert_eq!(ranges[2], 10..=12);

        assert_eq!(ingredients.len(), 3);
        assert_eq!(ingredients[0], 4);
        assert_eq!(ingredients[1], 5);
        assert_eq!(ingredients[2], 6);
    }

    #[test]
    fn filters_ingredients_in_range_correctly() {
        let ranges = vec![
            1..=3,
            5..=7,
        ];

        let ingredients = vec![
            2,
            4,
            5,
            8,
        ];

        let fresh: Vec<&u64> = ingredients_in_range(&ranges, &ingredients).collect();

        assert_eq!(fresh.len(), 2);
        assert_eq!(*fresh[0], 2);
        assert_eq!(*fresh[1], 5);
    }

    #[test]
    fn returns_0_for_empty_ranges() {
        assert_eq!(
            distinct_covered_count(&vec![]),
            0
        );
    }

    #[test]
    fn returns_range_size_for_one_range() {
        assert_eq!(
            distinct_covered_count(&vec![
                1..=2
            ]),
            2
        );
    }

    #[test]
    fn returns_total_range_size_for_two_non_overlapping_ranges() {
        assert_eq!(
            distinct_covered_count(&vec![
                1..=2,
                3..=4,
            ]),
            4
        );
    }

    #[test]
    fn returns_total_range_size_for_two_partially_overlapping_ranges() {
        assert_eq!(
            distinct_covered_count(&vec![
                1..=3,
                3..=4,
            ]),
            4
        );
    }

    #[test]
    fn returns_total_range_size_for_totally_overlapping_range() {
        assert_eq!(
            distinct_covered_count(&vec![
                1..=4,
                3..=4,
            ]),
            4
        );
    }

    #[test]
    fn returns_total_range_size_for_test_data() {
        assert_eq!(
            distinct_covered_count(&vec![
                3..=5,
                10..=14,
                16..=20,
                12..=18,
            ]),
            14
        );
    }
}