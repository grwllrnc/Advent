use itertools::Itertools;
use std::io::Error;
use utils::read_text_file;

fn repeated_at_least_twice(digits: &Vec<char>) -> bool {
    let len = digits.len();

    if len % 2 == 0 && (repeated_twice(digits) || compare_chunks(digits, 2)) {
        return true;
    }

    if len % 3 == 0 && compare_chunks(digits, 3) {
        return true;
    }

    if compare_chunks(digits, 1) {
        return true;
    }

    false
}

fn compare_chunks(digits: &Vec<char>, chunk_size: usize) -> bool {
    if digits.len() > chunk_size && digits.len() % chunk_size == 0 {
        let mut chunks = digits.chunks(chunk_size);
        let first = chunks.nth(0);
        
        while chunks.len() > 0 {
            let chunk = chunks.next();
            if chunk != first {
                return false;
            }
        }
        return true;
    }

    false
}

fn repeated_twice(digits: &Vec<char>) -> bool {
    let len = digits.len();
    if len % 2 == 0 {
        if digits[..len / 2] == digits[len / 2..] {
            return true;
        }
    }
    false
}

pub fn check_product_ids(input: &str) -> Result<(Vec<u64>, Vec<u64>), Error> {
    let data = read_text_file(input)?;
    let ranges = data[0]
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut invalid_repeated_twice = vec![];
    let mut invalid_repeated_at_least_twice = vec![];

    for range in ranges.into_iter() {
        let (start, end) = range
            .split("-")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple::<(u64, u64)>()
            .unwrap();
        for id in start..=end {
            let string_id = id.to_string();
            let id_len = string_id.len();
            let digits = string_id.chars().collect::<Vec<char>>();
            if id_len % 2 == 0 && repeated_twice(&digits) {
                invalid_repeated_twice.push(id);
            }
            if repeated_at_least_twice(&digits) {
                invalid_repeated_at_least_twice.push(id);
            }
        }
    }
    Ok((invalid_repeated_twice, invalid_repeated_at_least_twice))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_twice() {
        let ranges = [
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        let mut invalid_ids: Vec<u64> = vec![];

        for range in ranges.into_iter() {
            let (start, end) = range
                .split("-")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap();
            for id in start..=end {
                let digits = id.to_string().chars().collect::<Vec<char>>();
                let check = repeated_twice(&digits);
                if check {
                    invalid_ids.push(id);
                }
            }
        }
        assert_eq!(
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859],
            invalid_ids
        );
        let sum = invalid_ids.into_iter().sum::<u64>();
        assert_eq!(1227775554, sum);
    }

    #[test]
    fn test_compare_chunks() {
        let cases: Vec<Vec<char>> = vec!(
            vec!['1', '2'],
            vec!['9', '9', '9'],
            vec!['5', '6', '5', '6', '5', '6'],
            vec!['5', '6', '5', '5', '6', '5', '5', '6', '5',],
            vec!['2', '1', '2', '1', '2', '1', '2', '1', '2', '1']
        );
        let mut results = vec![];
        let mut even_results = vec![];
        let mut odd_results = vec![];

        for case in cases.into_iter() {
            results.push(compare_chunks(&case, 1));
            even_results.push(compare_chunks(&case, 2));
            odd_results.push(compare_chunks(&case, 3));
        }

        assert_eq!(vec![false, true, false, false, false], results);
        assert_eq!(vec![false, false, true, false, true], even_results);
        assert_eq!(vec![false, false, false, true, false], odd_results);
    }

    #[test]
    fn test_repeated_at_least_twice() {
        let ranges = [
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];

        let mut invalid_ids: Vec<u64> = vec![];

        for range in ranges.into_iter() {
            let (start, end) = range
                .split("-")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap();

            for id in start..=end {
                let digits = id.to_string().chars().collect::<Vec<char>>();
                let check = repeated_at_least_twice(&digits);

                if check {
                    invalid_ids.push(id);
                }
            }
        }
        assert_eq!(
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ],
            invalid_ids
        );
    }
}
