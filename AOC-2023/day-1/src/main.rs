use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Exctract numerical or number words from text in order they appeared.
///
/// Return: (i32, i32)
fn find_and_sort_number_words(input: &str) -> Option<String> {
    let numbers_map = [
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];
    
    let mut found_numbers = Vec::new();

    // Search for both numeric characters and words
    for &(num_str, word) in &numbers_map {
        let mut start = 0;
        while let Some(pos) = input[start..].find(num_str) {
            let actual_pos = start + pos;
            found_numbers.push((actual_pos, num_str));
            start = actual_pos + num_str.len();
        }

        let mut start = 0;
        while let Some(pos) = input[start..].find(word) {
            let actual_pos = start + pos;
            found_numbers.push((actual_pos, num_str));
            start = actual_pos + word.len();
        }
    }

    // Sort by the position where the number was found
    found_numbers.sort_by_key(|&(pos, _)| pos);

    // Directly return the result based on the number of elements found
    if found_numbers.is_empty() {
        None
    } else if found_numbers.len() == 1 {
        let num = found_numbers[0].1;
        let concat = concat_slices(num, num);
        Some(concat)
    } else {
        let first_num = found_numbers[0].1;
        let last_num = found_numbers.last().unwrap().1;
        let concat = concat_slices(first_num, last_num);
        Some(concat)
    }
}

fn concat_slices(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

fn sum_vector(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn string_to_i32(s: String) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn process_file(file_name: &str) -> Result<i32, Error> {
    let lines = read_file(file_name)?;
    let mut results: Vec<i32> = Vec::new();
    for line in lines {
        let num_str = find_and_sort_number_words(&line).expect("Bad concatenated number string");
        match string_to_i32(num_str) {
            Ok(content) => results.push(content),
            Err(_) => results.push(0),
        };
    }
    let result_sum = sum_vector(results);
    Ok(result_sum)
}

fn main() {
    match process_file("input.txt") {
        Ok(solution) => println!("The sum of the proccessed input is {}", solution),
        Err(_) => println!("Error processing input"),
    }  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_file() {
        match process_file("test_input.txt") {
            Ok(num) => {
                assert_eq!(num, 77);
            },
            Err(_) => panic!(),
        };
    }

    #[test]
    fn test_single_number_word() {
        let input = "three";
        let num_str = find_and_sort_number_words(input);
        assert_eq!(num_str, Some(String::from("3")));
    }

    #[test]
    fn test_multiple_number_words() {
        let input = "two5sixthree";
        let num_str = find_and_sort_number_words(input);
        assert_eq!(num_str, Some(String::from("23")));
    }

    #[test]
    fn test_no_number_words() {
        let input = "hello";
        let num_str = find_and_sort_number_words(input);
        assert_eq!(num_str, None);
    }

    #[test]
    fn test_numeric_and_word_numbers() {
        let input = "one7twofive";
        let num_str = find_and_sort_number_words(input);
        assert_eq!(num_str, Some(String::from("15")));
    }

    #[test]
    fn test_fucky_string() {
        let input = "twone";
        let num_str = find_and_sort_number_words(input);
        assert_eq!(num_str, Some(String::from("21")));
    }
}
