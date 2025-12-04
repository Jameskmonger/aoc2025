pub fn day_04(input: &str, part: u8) {
    let width = input.lines().nth(0).unwrap().len() as u8;
    let height = input.lines().count() as u8;

    let rolls: Vec<bool> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c == '@'))
        .collect();

    if part == 1 {
        let sum = get_accessible_roll_count(&rolls, width, height);
        println!("total accessible: {}", sum);
    } else if part == 2 {
        let total_removed = remove_accessible_rolls(rolls, width, height);
        println!("total removed: {}", total_removed);
    }
}

fn remove_accessible_rolls(mut rolls: Vec<bool>, width: u8, height: u8) -> u32 {
    let mut total_removed = 0;

    while let Some(count) = remove_accessible_layer(&mut rolls, width, height) {
        total_removed += count;
        if count == 0 {
            break;
        }
    }

    total_removed
}

fn remove_accessible_layer(rolls: &mut Vec<bool>, width: u8, height: u8) -> Option<u32> {
    let accessible: Vec<usize> = (0..rolls.len())
        .filter(|&i| is_accessible(rolls, i as u32, width, height))
        .collect();

    accessible.iter().for_each(|&i| rolls[i] = false);
    
    Some(accessible.len() as u32)
}

fn is_accessible(roll_map: &Vec<bool>, idx: u32, width: u8, height: u8) -> bool {
    roll_map[idx as usize]
    && get_adjacent_roll_count(roll_map, idx as u32, width, height) < 4
}

fn get_accessible_roll_count(roll_map: &Vec<bool>, width: u8, height: u8) -> u32 {
    (0..roll_map.len())
        .filter(|i| is_accessible(roll_map, *i as u32, width, height))
        .count() as u32
}

fn get_adjacent_roll_count(roll_map: &Vec<bool>, idx: u32, width: u8, height: u8) -> u8 {
    get_adjacent_indices(idx, width, height)
        .iter()
        .filter(|&&idx| roll_map[idx as usize])
        .count() as u8
}

fn get_adjacent_indices(idx: u32, width: u8, height: u8) -> Vec<u32> {
    let x = idx % width as u32;
    let y = idx / width as u32;

    [-1, 0, 1]
        .iter()
        .flat_map(|&dy| {
            [-1, 0, 1]
                .iter()
                .filter_map(move |&dx| {
                    if dx == 0 && dy == 0 {
                        return None;
                    }

                    let nx = (x as i32).checked_add(dx)?;
                    let ny = (y as i32).checked_add(dy)?;

                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        return None;
                    }

                    Some(ny as u32 * width as u32 + nx as u32)
                })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::{
        get_adjacent_indices,
        get_adjacent_roll_count
    };

    #[test]
    fn gets_adjacent_indices_for_inner_idx() {
        // 11 is (1, 1) on a 10x10 grid
        let idx = 11;

        let result = get_adjacent_indices(idx, 10, 10);

        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));

        assert!(result.contains(&10));
        // our tile
        assert!(result.contains(&12));
        
        assert!(result.contains(&20));
        assert!(result.contains(&21));
        assert!(result.contains(&22));

        assert_eq!(result.len(), 8);
    }

    
    #[test]
    fn doesnt_include_out_of_bounds_adjacent_up_or_left() {
        // 0 is (0, 0) on a 10x10 grid
        let idx = 0;

        let result = get_adjacent_indices(idx, 10, 10);

        assert!(result.contains(&1));
        assert!(result.contains(&10));
        assert!(result.contains(&11));
        
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn gets_adjacent_roll_count_all_true() {
        let roll_map = vec![
            true, true, true,
            true, true, true,
            true, true, true,
        ];

        let result = get_adjacent_roll_count(&roll_map, 4, 3, 3);

        assert_eq!(result, 8);
    }

    #[test]
    fn gets_adjacent_roll_count_all_false() {
        let roll_map = vec![
            false, false, false,
            false, false, false,
            false, false, false,
        ];

        let result = get_adjacent_roll_count(&roll_map, 4, 3, 3);

        assert_eq!(result, 0);
    }

    #[test]
    fn gets_adjacent_roll_count_mixed() {
        let roll_map = vec![
            false, true, false,
            false, false, true,
            true, false, true,
        ];

        let result = get_adjacent_roll_count(&roll_map, 4, 3, 3);

        assert_eq!(result, 4);
    }
}