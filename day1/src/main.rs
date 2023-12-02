use std::fs;

fn main() {
    let value_in_letters = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let contents: i64 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|info| {
            let mut new_info = String::from(info);

            for (idx, num) in value_in_letters.into_iter().enumerate() {
                let test: Vec<_> = info.match_indices(num).collect();
                for found in test.iter() {
                    new_info.replace_range(found.0..found.0 + 1, &(idx + 1).to_string());
                }
            }
            new_info
        })
        .map(|info| {
            let mut first_number: char = '0';
            let mut end_number: char = '0';

            for item in info.chars() {
                if item.to_string().parse::<i64>().is_ok() {
                    first_number = item;
                    break;
                };
            }
            for item in info.chars().rev() {
                if item.to_string().parse::<i64>().is_ok() {
                    end_number = item;
                    break;
                };
            }
            format!("{}{}", first_number, end_number)
                .parse::<i64>()
                .unwrap()
        })
        .sum();

    println!("{}", contents);
}
