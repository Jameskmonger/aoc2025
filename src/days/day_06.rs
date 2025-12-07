pub fn day_06(input: &str, part: u8) {
    let parser = if part == 1 { parse_input_pt1 } else { parse_input_pt2 };

    let results = parser(input)
        .iter()
        .map(|e| solve_equation(e))
        .fold(0, |acc, cur| acc + cur);

    println!("total of all results: {}", results);
}

type Equation = (Vec<u16>, Operator);

fn parse_input_pt1(input: &str) -> Vec<Equation>{
    let mut result: Vec<Equation> = 
        input
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|o| (vec![], if o == "*" { Operator::Multiply } else { Operator::Sum }))
            .collect();

    for line in input.lines().take(input.lines().count() - 1) {
        for (i, digit) in line.split_whitespace().map(|d| d.parse::<u16>().unwrap()).enumerate() {
            result[i].0.push(digit);
        }
    }

    result
}

fn parse_input_pt2(input: &str) -> Vec<Equation>{
    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let h = rows.len();
    let w = rows[0].len();
    for r in &rows {
        assert_eq!(r.len(), w, "All lines must be the same width");
    }

    let columns: Vec<Vec<char>> = (0..rows[0].len())
        .map(|c|
            rows
                .iter()
                .map(|r| r[c])
                .collect())
        .collect();

    // split the columns into groups, separated when a column contains
    // whitespace in all rows
    let mut column_groups: Vec<Vec<usize>> = Vec::new();
    let mut current_column_group: Vec<usize> = Vec::new();
    for x in 0..w {
        let is_empty_column = columns[x].iter().all(|&c| c == ' ');

        if is_empty_column {
            if !current_column_group.is_empty() {
                column_groups.push(current_column_group);
                current_column_group = vec![];
            }
        } else {
            current_column_group.push(x);
        }
    }
    if !current_column_group.is_empty() {
        column_groups.push(current_column_group);
    }

    column_groups
        .into_iter()
        .map(|idxs| {
            let operator = match columns[idxs[0]][h - 1] {
                '*' => Operator::Multiply,
                '+' => Operator::Sum,
                _ => panic!("could not find operator in column group"),
            };

            let mut nums: Vec<u16> = idxs
                .iter()
                .map(|&i| {
                    // get all digits in this column, join them into a single string and parse
                    columns[i][0..h - 1]
                        .iter()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u16>()
                        .unwrap_or(0)
                })
                .collect();

            // not strictly necessary but their example goes right-to-left
            nums.reverse();

            (nums, operator)
        })
        .collect()
}

fn solve_equation((numbers, operator): &Equation) -> u64 {
    numbers
        .iter()
        .fold(
            if operator == &Operator::Multiply { 1 } else { 0 },
            |acc, &cur| if operator == &Operator::Multiply { acc * cur as u64 } else { acc + cur as u64 })
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Multiply
}

#[cfg(test)]
mod test {
    use super::{
        Equation,
        Operator,
        parse_input_pt1,
        parse_input_pt2,
        solve_equation,
    };

    #[test]
    fn parses_input_pt1_correctly() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let result = parse_input_pt1(input);

        let expected: Vec<Equation> = vec![
            (vec![123, 45, 6], Operator::Multiply),
            (vec![328, 64, 98], Operator::Sum),
            (vec![51, 387, 215], Operator::Multiply),
            (vec![64, 23, 314], Operator::Sum),
        ];

        assert_eq!(
            expected,
            result,
        );
    }

    #[test]
    fn parses_input_pt2_correctly() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let result = parse_input_pt2(input);

        let expected: Vec<Equation> = vec![
            (vec![356, 24, 1], Operator::Multiply),
            (vec![8, 248, 369], Operator::Sum),
            (vec![175, 581, 32], Operator::Multiply),
            (vec![4, 431, 623], Operator::Sum),
        ];

        assert_eq!(
            expected,
            result,
        );
    }

    #[test]
    fn solves_sum_equation() {
        let equation: Equation = (
            vec![1, 2, 3],
            Operator::Sum
        );

        let result = solve_equation(&equation);

        assert_eq!(
            6,
            result,
        );
    }

    #[test]
    fn solves_multiply_equation() {
        let equation: Equation = (
            vec![1, 2, 3],
            Operator::Multiply
        );

        let result = solve_equation(&equation);

        assert_eq!(
            6,
            result,
        );
    }
}