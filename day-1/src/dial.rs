use std::num::ParseIntError;

const DIAL_LEN: i32 = 100;

pub fn dial(pos: &i32, direction: &str, clicks: &str) -> Result<(i32, i32), ParseIntError> {
    let clicks = clicks.parse::<i32>()?;
    let full_rotation: i32 = clicks / DIAL_LEN;
    let remaining_clicks: i32 = clicks % DIAL_LEN;
    let mut crossed_zeros: i32 = 0;
    let new_pos: i32;

    if direction == "R" {
        new_pos = (*pos + clicks).rem_euclid(DIAL_LEN);
        crossed_zeros += full_rotation;

        if *pos != 0 && remaining_clicks > DIAL_LEN - *pos {
            crossed_zeros += 1;
        }
    } else {
        new_pos = (*pos - clicks).rem_euclid(DIAL_LEN);
        crossed_zeros += full_rotation;

        if *pos != 0 && *pos - remaining_clicks < 0 {
            crossed_zeros += 1;
        }
    }

    Ok((crossed_zeros, new_pos))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial() {
        let input = [
            ("L", "68"),
            ("L", "30"),
            ("R", "48"),
            ("L", "5"),
            ("R", "60"),
            ("L", "55"),
            ("L", "1"),
            ("L", "99"),
            ("R", "14"),
            ("L", "82")
        ];
        let mut zeros: i32 = 0;
        let mut crossed_zeros: i32 = 0;
        let mut pos: i32 = 50;

        for i in 0..10 {
            let (direction, clicks) = input[i]; 
            let result = dial(&pos, &direction, &clicks);
            if result.is_ok() {
                let (zero_count, new_pos) = result.unwrap();
                if new_pos == 0 {
                    zeros += 1;
                }
                
                crossed_zeros += zero_count;
                
                println!("Position {} {}{} new position {}", pos, direction, clicks, new_pos);
                println!("Crossed zeros: {}", zero_count);
                pos = new_pos;
            }
        }
        println!("zeros {}, crossed_zeros {}", zeros, crossed_zeros);
        assert_eq!(zeros + crossed_zeros, 6);
    }

    #[test]
    fn test_dial_2() {
        let (direction, clicks) = ("L", "1000");
        let result = dial(&50, &direction, &clicks);
            
        println!("result {:?}", &result.as_ref().unwrap());
        assert_eq!(result, Ok((10, 50)));
    }
}