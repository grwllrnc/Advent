use utils::read_text_file;

mod paperroll;

fn main() {
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt");
    let lines = read_text_file(file).unwrap();
    let mut input = lines.into_iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut counts: Vec<i32> = vec![];
    let mut has_accessible = true;

    let mut accessible_count = 0;

    for i in 0..input.len() {
        for c in 0..input[i].len() {
            if paperroll::check_ajacent_position(&input, i, c) == Some(true) {
                accessible_count += 1;         // count accessible paper rolls
            }
        }
    }

    while has_accessible == true {
        let mut accessible_paper_roll_count = 0;
    
        for i in 0..input.len() {
            for c in 0..input[i].len() {
                if paperroll::check_ajacent_position(&input, i, c) == Some(true) {
                    accessible_paper_roll_count += 1;         // count accessible paper rolls
                    input[i][c] = '.'; // remove paper roll
                }
            }
        }
        if accessible_paper_roll_count != 0 {
            counts.push(accessible_paper_roll_count);
        } else {
            has_accessible = false;
        }

    }
    println!("Number of accessible paper rolls (first part): {}", &accessible_count);
    println!("Total number of paper rolls removed by the Elves and their forklifts (part two): {}", &counts.iter().sum::<i32>());
}
