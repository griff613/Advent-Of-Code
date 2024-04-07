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

fn solve_day1(parts: Vec<&str>) -> bool {
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

fn solve_day2(parts: Vec<&str>) -> {
    
}

fn process_input(file_name: &str) -> HashMap<i32, bool> {
    

    // Keep a map of the game number and whether or not it was possible
    let mut game_map: HashMap<i32, bool> = HashMap::new();
    let mut game_number = 1;
    let lines = read_file("input.txt").expect("parse error");
    for line in lines {
    
        // get rid of 'game #:'
        let stripped_line = match line.find(':') {
            Some(index) => &line[index + 1..].trim(),
            None => &line[..],
        };

        let parts: Vec<&str> = stripped_line.split(';').collect();
        let game_possible = solve_day1(parts);
        game_map.insert(game_number, game_possible);
        game_number += 1;
    }
    game_map
}

fn main() {
    let game_map = process_input("test_input.txt");
    let mut possible_counter = 0;
    for (key, value) in &game_map {
        if *value {
            possible_counter += key;
        }
    }

    println!("Sum of possible game #'s: {}", possible_counter);
}
