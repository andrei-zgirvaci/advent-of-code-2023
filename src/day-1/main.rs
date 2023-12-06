use std::collections::HashMap;
use std::fs::read_to_string;

fn get_number_from_line(line: &str) -> usize {
    const NUMBER_STRINGS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut index_numbers: HashMap<usize, usize> = HashMap::new();

    for number in NUMBER_STRINGS {
        let index_values: Vec<usize> = line.match_indices(number).map(|(i, _)| i).collect();

        for index in index_values {
            let parsed_number: usize = match number {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => number.parse().unwrap_or(0),
            };
            index_numbers.insert(index, parsed_number);
            println!("{}: {}", index, number);
        }
    }

    if index_numbers.len() == 0 {
        return 0;
    }

    let min_key: usize = index_numbers.keys().min().cloned().unwrap();
    let max_key: usize = index_numbers.keys().max().cloned().unwrap();

    let first_digit = index_numbers.get(&min_key).cloned().unwrap();
    let last_digit = index_numbers.get(&max_key).cloned().unwrap();

    println!("{}: {}, {}: {}", min_key, first_digit, max_key, last_digit);

    return first_digit * 10 + last_digit;
}

fn main() {
    const FILE_PATH: &str = "src/day-1/input";

    let contents: String = read_to_string(FILE_PATH).unwrap();

    let mut sum: usize = 0;
    for line in contents.lines() {
        println!("{}", line);

        let number: usize = get_number_from_line(line);

        println!("number: {}", number);

        sum += number;
    }

    println!("sum: {}", sum);
}
