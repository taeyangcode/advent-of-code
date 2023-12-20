#![allow(dead_code)]

const ASCII_RANGE: std::ops::RangeInclusive<u8> = 49..=57;
const ASCII_NUMBER_OFFSET: u8 = 48;
const NEW_LINE: u8 = 10;

#[inline]
fn handle_number(number: u8, last_number: &mut (u8, u8)) -> () {
    if last_number.0 == 0 {
        last_number.0 = number - ASCII_NUMBER_OFFSET;
        return;
    }
    last_number.1 = number - ASCII_NUMBER_OFFSET;
}

#[inline]
fn handle_new_line(calibration_sum: &mut u32, last_number: &mut (u8, u8)) -> () {
    match last_number.1 == 0 {
        true => *calibration_sum += (last_number.0 * 10 + last_number.0) as u32,
        false => *calibration_sum += (last_number.0 * 10 + last_number.1) as u32,
    };

    *last_number = (0, 0);
}

fn parse_letter(letter: u8, calibration_sum: &mut u32, last_number: &mut (u8, u8)) {
    match letter {
        _ if ASCII_RANGE.contains(&letter) => handle_number(letter, last_number),
        NEW_LINE => handle_new_line(calibration_sum, last_number),
        _ => {},
    };
}

pub fn solution(path: &str) -> u32 {
    let file_contents: Vec<u8> = std::fs::read(path).expect("ERROR: invalid path");

    let mut calibration_sum: u32 = 0;
    let mut last_number: (u8, u8) = (0, 0);

    file_contents
        .into_iter()
        .for_each(|letter: u8| parse_letter(letter, &mut calibration_sum, &mut last_number));

    return calibration_sum;
}
