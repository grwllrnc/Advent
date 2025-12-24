mod checks;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt");
    let result = checks::check_product_ids(path);

    match result {
        Ok((repeated_twice, repeated_at_least_twice)) => {
            println!(
                "Sum of invalid ids (repeated twice): {}",
                repeated_twice.iter().sum::<u64>()
            );
            println!(
                "Sum of invalid ids (repeated at least twice): {}",
                repeated_at_least_twice.iter().sum::<u64>()
            );
        },
        Err(e) => println!("{}", e)
    }
}
