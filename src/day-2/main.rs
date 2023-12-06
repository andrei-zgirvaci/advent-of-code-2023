use regex::Regex;
use std::fs::read_to_string;

struct Game {
    red_cubes_count: usize,
    green_cubes_count: usize,
    blue_cubes_count: usize,
    power_minimum_set_cubes: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            red_cubes_count: 0,
            green_cubes_count: 0,
            blue_cubes_count: 0,
            power_minimum_set_cubes: 0,
        }
    }

    fn set_red_cubes_count(&mut self, new_count: usize) {
        self.red_cubes_count = new_count;
    }

    fn set_green_cubes_count(&mut self, new_count: usize) {
        self.green_cubes_count = new_count;
    }

    fn set_blue_cubes_count(&mut self, new_count: usize) {
        self.blue_cubes_count = new_count;
    }

    fn calc_power_minimum_set_cubes(&mut self) {
        self.power_minimum_set_cubes =
            self.red_cubes_count * self.green_cubes_count * self.blue_cubes_count;
    }

    fn get_red_cubes_count(&self) -> usize {
        return self.red_cubes_count;
    }

    fn get_green_cubes_count(&self) -> usize {
        return self.green_cubes_count;
    }

    fn get_blue_cubes_count(&self) -> usize {
        return self.blue_cubes_count;
    }

    fn get_power_minimum_set_cubes(&self) -> usize {
        return self.power_minimum_set_cubes;
    }
}

fn main() {
    const FILE_PATH: &str = "src/day-2/input";

    let contents: String = read_to_string(FILE_PATH).unwrap();

    let mut games: Vec<Game> = Vec::new();

    const COLORS: [&str; 3] = ["red", "green", "blue"];

    for line in contents.lines() {
        let mut game = Game::new();

        let game_sets_lines: Vec<&str> = line.split(';').collect();

        for color in COLORS {
            let mut max_cubes_count: usize = 0;

            for game_set_line in &game_sets_lines {
                let regex_pattern: String = format!(r"(\d+) {}", color);

                let re: regex::Regex = Regex::new(&regex_pattern).unwrap();

                if let Some(mat) = re.captures(game_set_line) {
                    let cubes_count: usize = mat.get(1).unwrap().as_str().parse::<usize>().unwrap();

                    if cubes_count > max_cubes_count {
                        max_cubes_count = cubes_count;
                    }
                }
            }

            match color {
                "red" => game.set_red_cubes_count(max_cubes_count),
                "green" => game.set_green_cubes_count(max_cubes_count),
                "blue" => game.set_blue_cubes_count(max_cubes_count),
                _ => println!("It's something else"),
            }
        }

        game.calc_power_minimum_set_cubes();

        games.push(game)
    }

    const RED_CUBES_AVAILABLE: usize = 12;
    const GREEN_CUBES_AVAILABLE: usize = 13;
    const BLUE_CUBES_AVAILABLE: usize = 14;

    println!(
        "Available cubs: {} red, {} green, {} blue",
        RED_CUBES_AVAILABLE, GREEN_CUBES_AVAILABLE, BLUE_CUBES_AVAILABLE
    );

    let mut possible_games_ids: Vec<usize> = Vec::new();
    let mut sum_power_minimum_set_cubes: usize = 0;

    for (index, game) in games.iter().enumerate() {
        let real_index: usize = index + 1;

        if game.get_red_cubes_count() <= RED_CUBES_AVAILABLE
            && game.get_green_cubes_count() <= GREEN_CUBES_AVAILABLE
            && game.get_blue_cubes_count() <= BLUE_CUBES_AVAILABLE
        {
            possible_games_ids.push(real_index);
        }

        let power_minimum_set_cubes: usize = game.get_power_minimum_set_cubes();

        println!(
            "Game {}: {} red cubes, {} green cubes, {} blue cubes, min power set: {}",
            real_index,
            game.get_red_cubes_count(),
            game.get_green_cubes_count(),
            game.get_blue_cubes_count(),
            power_minimum_set_cubes
        );

        sum_power_minimum_set_cubes += power_minimum_set_cubes;
    }

    println!("Possible games ids: {:?}", possible_games_ids);

    let possible_games: usize = possible_games_ids.iter().sum();

    println!("Possible games result: {}", possible_games);

    println!("Sum min power set: {}", sum_power_minimum_set_cubes)
}
