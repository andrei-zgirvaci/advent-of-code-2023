use std::fs::read_to_string;

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    const NUMBER_STRINGS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut numbers: Vec<u32> = Vec::new();

    for character in line.chars() {
        if let Some(digit) = character.to_digit(10) {
            numbers.push(digit);
            println!("{}", digit);
        }
    }

    return numbers;
}

fn first_1() {
    const FILE_PATH: &str = "src/1_input";

    let contents: String = read_to_string(FILE_PATH).unwrap();

    let mut input_lines: Vec<&str> = Vec::new();
    for line in contents.lines() {
        input_lines.push(line);
    }

    let mut sum: u32 = 0;
    for line in input_lines {
        println!("{}", line);

        let numbers: Vec<u32> = get_numbers_from_line(line);

        let first_digit = numbers.first().unwrap_or(&0);
        let last_digit = numbers.last().unwrap_or(&0);

        let number: u32 = first_digit * 10 + last_digit;

        println!("number: {}", number);

        sum += number;
    }

    println!("sum: {}", sum);
}

fn main() {
    first_1();
}
