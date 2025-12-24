use utils::{split, read_text_file};
mod dial;

fn main() -> () {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt");
    let lines = read_text_file(path);
    let instructions: Vec<(String, String)> = lines.unwrap().into_iter().map(|line| split(r"(\w)(.+)", line).unwrap()).collect();
    let mut dialed_zeros: i32 = 0;
    let mut crossed_zeros: i32 = 0;
    let mut pos: i32 = 50;

    for i in 0..instructions.len() {
        let (direction, clicks) = &instructions[i]; 
        let result = dial::dial(&pos, &direction, &clicks);
        if result.is_ok() {
            let (zero_count, new_pos) = result.unwrap();
            if new_pos == 0 {
                dialed_zeros += 1;
            }
            
            crossed_zeros += zero_count;
            pos = new_pos;
        }
    }
    println!("Dialed zeros: {}", dialed_zeros);
    println!("Crossed zeros: {}", crossed_zeros);
    println!("Total zeros: {}", dialed_zeros + crossed_zeros);
}