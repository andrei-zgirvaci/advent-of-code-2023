use std::fs::read_to_string;

fn main() {
    const FILE_PATH: &str = "src/test_input";

    let contents: String = read_to_string(FILE_PATH).unwrap();

    println!("{:?}", contents)
}
