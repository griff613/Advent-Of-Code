/// Snow Island
/// small bag of cubes, cubes are either red, green, or blue.
///elf hides a secret number of cubes of each color in the bag,
/// goal is to figure out information about the number of cubes.

/// Which games would have been possible if the bag contained
///  **only 12 red cubes, 13 green cubes, and 14 blue cubes***

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn solve_part1(parts: Vec<&str>) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut game_possible = true;

    for part in parts {
        let reveals: Vec<&str> = part.split(',').collect(); 
        for reveal in reveals {
            let values: Vec<&str> = reveal.split_whitespace().collect();
            let num_i32: Result<i32, _> = values[0].parse();
            let color = values[1];
            match num_i32 {
                Ok(n) => {
                    if(color == "red" && n > max_red) ||
                    (color == "green" && n > max_green) ||
                    (color == "blue" && n > max_blue) {
                        game_possible = false;
                    }
                }
                Err(_) => println!("Error checking colors"),
            }                
            println!("{:?}", reveal.trim());
        }
    }
    game_possible
}

fn solve_part2(parts: Vec<&str>) -> i32 {
    let mut color_maxes: HashMap<&str, i32> = HashMap::new();
    color_maxes.insert("red", 0);
    color_maxes.insert("green", 0);
    color_maxes.insert("blue", 0);
    // Get the max of each color for this game
    for part in parts {
        let reveals: Vec<&str> = part.split(',').collect();
        for reveal in reveals {
            let values: Vec<&str> = reveal.split_whitespace().collect();
            let num_i32: Result<i32, _> = values[0].parse();
            let color = values[1];
            match num_i32 {
                Ok(n) => {
                    if let Some(value) = color_maxes.get(color) {
                        if n > *value {
                            color_maxes.insert(color, n);
                        }
                    }                    
                }
                Err(_) => println!("couldn't parse number into i32")
            }
        }
    }

    let mut  game_score = 1;
    if let Some(max_red) = color_maxes.get("red") {
        game_score *= max_red;
    }
    if let Some(max_green) = color_maxes.get("green") {
        game_score *= max_green;
    }
    if let Some(max_blue) = color_maxes.get("blue") {
        game_score *= max_blue;
    }

    println!("{}", game_score);
    game_score

}

fn process_input_part2(file_name: &str) -> i32 {
    
    // Keep a map of the game number and whether or not it was possible
    let mut game_result = 0;

    let lines = read_file(file_name).expect("parse error");
    for line in lines {
    
        // get rid of 'game #:'
        let stripped_line = match line.find(':') {
            Some(index) => line[index + 1..].trim(),
            None => &line[..],
        };

        let parts: Vec<&str> = stripped_line.split(';').collect();
        let game_value = solve_part2(parts);
        game_result += game_value;
    }
    game_result
}

fn main() {
    let result = process_input_part2("input.txt");
    println!("Sum of powers={}", result);
}
