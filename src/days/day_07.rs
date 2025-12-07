use std::collections::{HashMap, HashSet};

pub fn day_07(input: &str, part: u8) {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();

    let beams: Vec<usize> = get_indices_of_char(
        first_line,
        'S'
    );

    let splitter_lines: Vec<Vec<usize>> = lines
        .map(|l| get_indices_of_char(l, '^'))
        .collect();

    let total = if part == 1 {
        get_total_split_count(&beams, &splitter_lines) as u64
    } else {
        get_total_timeline_count(beams[0], &splitter_lines)
    };

    println!("total {}", total);
}

fn get_indices_of_char(line: &str, char: char) -> Vec<usize> {
    line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| 
            if c == char {
                Some(i)
            } else {
                None
            }
        )
        .collect()
}

fn get_new_beam_positions(beams: &Vec<usize>, splitters: &Vec<usize>) -> (Vec<usize>, u32) {
    let mut new_beams: Vec<usize> = vec![];
    let mut split_count: u32 = 0;

    for beam in beams {
        if splitters.contains(beam) {
            new_beams.push(beam - 1);
            new_beams.push(beam + 1);
            split_count += 1;
        } else {
            new_beams.push(*beam);
        }
    }
    
    dedupe(&mut new_beams);

    (new_beams, split_count)
}

fn dedupe(v: &mut Vec<usize>) {
    let mut set = HashSet::new();

    v.retain(|&x| set.insert(x));
}

fn get_total_split_count(beams: &Vec<usize>, splitter_lines: &Vec<Vec<usize>>) -> u32 {
    let mut beams = beams.clone();
    let mut total = 0;
    
    for splitters in splitter_lines {
        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        beams = new_beams;
        total += split_count;
    }

    total
}

fn get_total_timeline_count(beam: usize, splitter_lines: &Vec<Vec<usize>>) -> u64 {
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    count_timelines_from(splitter_lines, &mut memo, beam, 0)
}

fn count_timelines_from(
    splitter_lines: &Vec<Vec<usize>>,
    memo: &mut HashMap<(usize, usize), u64>,
    beam: usize,
    row: usize,
) -> u64 {
    if row >= splitter_lines.len() {
        return 1;
    }

    // cache by (beam and row) so we don't recalculate
    if let Some(&cached) = memo.get(&(beam, row)) {
        return cached;
    }

    let splitters = &splitter_lines[row];
    let next_row = row + 1;

    let result = if splitters.contains(&beam) {
        let left = count_timelines_from(splitter_lines, memo, beam - 1, next_row);
        let right = count_timelines_from(splitter_lines, memo, beam + 1, next_row);

        left + right
    } else {
        count_timelines_from(splitter_lines, memo, beam, next_row)
    };

    memo.insert((beam, row), result);

    result
}

#[cfg(test)]
mod test {
    use super::{
        get_new_beam_positions,
        get_indices_of_char,
        get_total_split_count,
        get_total_timeline_count,
    };

    #[test]
    fn parses_start_point() {
        let input = ".......S.......";

        let result = get_indices_of_char(input, 'S');

        assert_eq!(
            result,
            vec![7],
        )
    }

    #[test]
    fn parses_splitters() {
        let input = ".^.^.^.^.^...^.";

        let result = get_indices_of_char(input, '^');

        assert_eq!(
            result,
            vec![1, 3, 5, 7, 9, 13],
        )
    }

    #[test]
    fn continues_single_beam_downwards_if_no_splitters() {
        let beams: Vec<usize> = vec![7];
        let splitters: Vec<usize> = vec![];

        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(new_beams, beams);
        assert_eq!(split_count, 0);
    }

    #[test]
    fn continues_single_beam_downwards_if_doesnt_hit_splitter() {
        let beams: Vec<usize> = vec![7];
        let splitters: Vec<usize> = vec![
            0, 1, 2, 3, 4, 5, 6,
            8, 9, 10, 11, 12, 13, 14
        ];

        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(new_beams, beams);
        assert_eq!(split_count, 0);
    }

    #[test]
    fn splits_single_beam() {
        let beams: Vec<usize> = vec![7];
        let splitters: Vec<usize> = vec![7];

        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(new_beams, vec![6, 8]);
        assert_eq!(split_count, 1);
    }

    #[test]
    fn continues_multiple_beams_downwards_if_no_splitters() {
        let beams: Vec<usize> = vec![4, 10];
        let splitters: Vec<usize> = vec![];
        
        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(new_beams, beams);
        assert_eq!(split_count, 0);
    }

    #[test]
    fn continues_multiple_beams_downwards_if_doesnt_hit_splitter() {
        let beams: Vec<usize> = vec![4, 10];
        let splitters: Vec<usize> = vec![7];
        
        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(new_beams, beams);
        assert_eq!(split_count, 0);
    }

    #[test]
    fn splits_multiple_beams_distinctly() {
        let beams: Vec<usize> = vec![1, 3, 4, 5, 7, 8, 10, 11, 13];
        let splitters: Vec<usize> = vec![1, 3, 5, 7, 9, 13];
        
        let (new_beams, split_count) = get_new_beam_positions(&beams, &splitters);

        assert_eq!(
            new_beams,
            vec![0, 2, 4, 6, 8, 10, 11, 12, 14],
        );
        assert_eq!(split_count, 5);
    }

    #[test]
    fn gets_total_split_count() {
        // data from part 1 example
        let splitter_groups: Vec<Vec<usize>> = vec![
            vec![],
            vec![7],
            vec![],
            vec![6, 8],
            vec![],
            vec![5, 7, 9],
            vec![],
            vec![4, 6, 10],
            vec![],
            vec![3, 5, 9, 11],
            vec![],
            vec![2, 6, 12],
            vec![],
            vec![1, 3, 5, 7, 9, 13],
            vec![],
        ];

        let total = get_total_split_count(
            &vec![7],
            &splitter_groups
        );

        assert_eq!(total, 21);
    }

    #[test]
    fn gets_total_timeline_count() {
        // data from part 1 example
        let splitter_groups: Vec<Vec<usize>> = vec![
            vec![],
            vec![7],
            vec![],
            vec![6, 8],
            vec![],
            vec![5, 7, 9],
            vec![],
            vec![4, 6, 10],
            vec![],
            vec![3, 5, 9, 11],
            vec![],
            vec![2, 6, 12],
            vec![],
            vec![1, 3, 5, 7, 9, 13],
            vec![],
        ];

        let total = get_total_timeline_count(
            7,
            &splitter_groups,
        );

        assert_eq!(total, 40);
    }
}