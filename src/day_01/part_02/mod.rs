#![allow(dead_code)]

const NEW_LINE: u8 = 10;
const ASCII_RANGE: std::ops::RangeInclusive<u8> = 49..=57;
const ASCII_NUMBER_OFFSET: u8 = 48;

static STRING_NUMBER: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_first_value(line: &[u8]) -> u32 {
    let line_length: usize = line.len();
    for (index, character) in line.into_iter().enumerate() {
        match *character {
            _ if ASCII_RANGE.contains(character) => return (character - ASCII_NUMBER_OFFSET) as u32,
            _ => {
                if let Some(position) = STRING_NUMBER.into_iter().position(|string: &str| line[index..line_length].starts_with(string.as_bytes())) {
                    return (position + 1) as u32;
                }
            },
        };
    }
    unreachable!("ERROR: no number found in line");
}

fn find_last_value(line: &[u8]) -> u32 {
    let line_length: usize = line.len();
    for (index, character) in line.into_iter().enumerate().rev() {
        match *character {
            _ if ASCII_RANGE.contains(character) => return (character - ASCII_NUMBER_OFFSET) as u32,
            _ => {
                if let Some(position) = STRING_NUMBER.into_iter().position(|string: &str| line[index..line_length].starts_with(string.as_bytes())) {
                    return (position + 1) as u32;
                }
            },
        };
    }
    unreachable!("ERROR: no number found in line");
}

fn calculate_line_value(line: &[u8]) -> u32 {
    return find_first_value(line) * 10 + find_last_value(line);
}

pub fn solution(path: &'static str) -> u32 {
    let file_contents: Vec<u8> = std::fs::read(path).expect("ERROR: invalid path");

    let mut line_start: usize = 0;
    file_contents
        .iter()
        .enumerate()
        .fold(0, |sum: u32, (index, letter): (usize, &u8)| {
            if *letter == NEW_LINE {
                let result: u32 = calculate_line_value(&file_contents[line_start..index]);
                line_start = index + 1;
                return sum + result;
            }
            sum
        })
}
