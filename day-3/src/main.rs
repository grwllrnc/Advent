use std::num::ParseIntError;
use utils::read_text_file;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt");
    let banks = read_text_file(path);
    match banks {
        Ok(banks) => {
            let mut max_charges_2_digits: Vec<u64> = vec!();
            let mut max_charges_12_digits: Vec<u64> = vec!();
            
            for raw_bank in banks.into_iter() {
                let bank = cast_into_u32_vec(raw_bank);
                let max_jol_2 = max_joltage(&bank, 2);
                let max_jol_12 = max_joltage(&bank, 12);
                if max_jol_2.is_ok() && max_jol_12.is_ok() {
                    max_charges_2_digits.push(max_jol_2.unwrap());
                    max_charges_12_digits.push(max_jol_12.unwrap());
                } else {
                    panic!("Error while max_joltage");
                }
            }
        
            let total_2: u64 = max_charges_2_digits.into_iter().sum();
            let total_12: u64 = max_charges_12_digits.into_iter().sum();
            println!("Total charge part 1 (2 digits): {}", total_2);
            println!("Total charge part 2 (12 digits): {}", total_12);
        },
        Err(e) => println!("Error: {}", e)
    }
}

fn cast_into_u32_vec(bank: String) -> Vec<u64> {
    bank
        .chars()
        .into_iter()
        .map(|c| c.to_string()
            .parse::<u64>()
            .unwrap()
        )
        .collect::<Vec<u64>>()
}

fn get_max(vec: &[u64]) -> (usize, u64) {
    let mut curr_max: u64 = 0;
    let mut max_idx: usize = 0;
    for i in 0..vec.len() {
        if curr_max < vec[i] {
            curr_max = vec[i];
            max_idx = i;
        }
    }
    (max_idx, curr_max)
}

fn max_joltage(bank: &Vec<u64>, digits: usize) -> Result<u64, ParseIntError> {
    let mut batteries: Vec<u64> = vec!();
    let mut places = digits;
    let mut start = 0;
    
    while batteries.len() < digits {
        let end = bank.len() - places;
        let (idx, digit) = get_max(&bank[start..=end]);
        batteries.push(digit);
        start = start + idx + 1;
        places -= 1;
    }

    let value = batteries.into_iter().fold(0u64, |acc, d| acc * 10 + d);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_into_u32_vec() {
        let raw_bank = "987654321111111";
        let bank = cast_into_u32_vec(raw_bank.to_string());
        assert_eq!(bank, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_get_max() {
        let sample = [
            "987",
            "654",
            "321",
            "111",
            "111",
        ];
        let result = sample.into_iter().map(|n| get_max(&cast_into_u32_vec(n.to_string())).1 ).collect::<Vec<u64>>();
        assert_eq!(result, [9,6,3,1,1]);
    }

    #[test]
    fn test_max_joltage() {
        let raw_bank = "234234234234278";
        let bank = cast_into_u32_vec(raw_bank.to_string());
        let max_jol = max_joltage(&bank, 2);
        assert_eq!(max_jol, Ok(78));
    }

    #[test]
    fn test_example_part_one() {
        let sample = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111"
        ];
        let mut charges = vec!();

        for raw_bank in sample.into_iter() {
            let bank = cast_into_u32_vec(raw_bank.to_string());
            let max_jol = max_joltage(&bank, 2);
            charges.push(max_jol.unwrap());
        }
        assert_eq!(charges.into_iter().sum::<u64>(), 357);
    }

    #[test]
    fn test_example_part_two() {
        let sample = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111"
        ];
        let mut charges = vec!();

        for raw_bank in sample.into_iter() {
            let bank = cast_into_u32_vec(raw_bank.to_string());
            let max_jol = max_joltage(&bank, 12);
            charges.push(max_jol.unwrap());
        }
        assert_eq!(charges.into_iter().sum::<u64>(), 3121910778619);
    }
}
