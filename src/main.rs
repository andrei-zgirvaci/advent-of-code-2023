use std::fs::read_to_string;

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

        let mut numbers: Vec<u32> = Vec::new();
        for character in line.chars() {
            if let Some(digit) = character.to_digit(10) {
                numbers.push(digit);
                println!("{}", digit);
            }
        }

        let mut number: u32 = 0;
        if let Some(first_digit) = numbers.first() {
            let last_digit = numbers.last().unwrap();

            number = first_digit * 10 + last_digit;
        }

        println!("number: {}", number);

        sum += number;
    }

    println!("sum: {}", sum);
}

fn first_2() {
    const FILE_PATH: &str = "src/1_input";

    let contents: String = read_to_string(FILE_PATH).unwrap();
}

fn main() {
    first_1();
}
