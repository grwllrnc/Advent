pub fn check_ajacent_position(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<bool> {
    if grid[row][col] == '.' {
        return None;
    } else {
        return Some(is_accessible(&grid, &row, &col));
    }
}

fn is_accessible(grid: &Vec<Vec<char>>, row: &usize, col: &usize) -> bool {
    let row_start = if *row == 0 { 0 } else { row - 1 };
    let row_end = if *row == grid.len() - 1 { row } else { &(row + 1) };
    let col_start = if *col == 0 { 0 } else { col - 1 };
    let col_end = if *col == grid[0].len() - 1 { col } else { &(col + 1) }; 
    let slice = grid[row_start..=*row_end]
        .iter()
        .map(|r| r[col_start..=*col_end].to_vec())
        .flatten()
        .fold(-1, |acc, p| { if p == '@' { acc + 1 } else { acc + 0 }});
    slice < 4
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_text_file;

    #[test]
    fn test_is_accessible() {
        let file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/sample_input.txt");
        let lines = read_text_file(file).unwrap();
        let input = lines.into_iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut count = 0;

        for i in 0..input.len() {
            for c in 0..input[i].len() {
                if check_ajacent_position(&input, i, c) == Some(true) {
                    count += 1;
                }
            }
        }

        assert_eq!(vec!['@', '@', '@', '.', '@', '.','@', '.', '@', '@'], input[1]);
        assert_eq!(check_ajacent_position(&input, 2, 6), Some(true));
        assert_eq!(check_ajacent_position(&input, 2, 3), Some(false));
        assert_eq!(check_ajacent_position(&input, 1, 3), None);
        assert_eq!(count, 13);
    }
}
