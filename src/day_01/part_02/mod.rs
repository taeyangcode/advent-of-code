const NEW_LINE: u8 = 10;
const ASCII_RANGE: std::ops::RangeInclusive<u8> = 49..=57;
const ASCII_NUMBER_OFFSET: u8 = 48;

mod front {
    pub fn t_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 {
            if &characters[0..4] == [b'h', b'r', b'e', b'e'] {
                return Some(3);
            }

            if remaining_length >= 2 {
                if &characters[0..2] == [b'w', b'o'] {
                    return Some(2);
                }
            }
        }
        return None;
    }

    pub fn f_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 3 {
            if &characters[0..3] == [b'o', b'u', b'r'] {
                return Some(4);
            }
            if &characters[0..3] == [b'i', b'v', b'e'] {
                return Some(5);
            }
        }
        return None;
    }

    pub fn s_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 {
            if &characters[0..4] == [b'e', b'v', b'e', b'n'] {
                return Some(7);
            }
            if remaining_length >= 2 {
                if &characters[0..2] == [b'i', b'x'] {
                    return Some(6);
                }
            }
        }
        return None;
    }

    pub fn o_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 2 && characters[0..2] == [b'n', b'e'] {
            return Some(1);
        }
        return None;
    }

    pub fn e_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 && characters[0..4] == [b'i', b'g', b'h', b't'] {
            return Some(8);
        }
        return None;
    }

    pub fn n_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 3 && characters[0..3] == [b'i', b'n', b'e'] {
            return Some(9);
        }
        return None;
    }
}

mod back {
    pub fn e_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 {
            if &characters[0..4] == [b'h', b'r', b'e', b'e'] {
                return Some(3);
            }
            if remaining_length >= 2 && characters[0..2] == [b'n', b'e'] {
                return Some(1);
            }
        }
        return None;
    }

    pub fn f_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 3 {
            if &characters[0..3] == [b'o', b'u', b'r'] {
                return Some(4);
            }
            if &characters[0..3] == [b'i', b'v', b'e'] {
                return Some(5);
            }
        }
        return None;
    }

    pub fn s_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 {
            if &characters[0..4] == [b'e', b'v', b'e', b'n'] {
                return Some(7);
            }
            if remaining_length >= 2 {
                if &characters[0..2] == [b'i', b'x'] {
                    return Some(6);
                }
            }
        }
        return None;
    }

    pub fn o_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 2 && characters[0..2] == [b'n', b'e'] {
            return Some(1);
        }
        return None;
    }

    pub fn e_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 4 && characters[0..4] == [b'i', b'g', b'h', b't'] {
            return Some(8);
        }
        return None;
    }

    pub fn n_number(characters: &[u8], remaining_length: usize) -> Option<u8> {
        if remaining_length >= 3 && characters[0..3] == [b'i', b'n', b'e'] {
            return Some(9);
        }
        return None;
    }
}

fn find_first_value(line: &[u8]) -> u32 {
    for (index, character) in line.into_iter().enumerate() {
        let remaining_length: usize = line.len() - index - 1;
        match character {
            _ if ASCII_RANGE.contains(character) => return (character - ASCII_NUMBER_OFFSET) as u32,
            b't' => {
                if let Some(first_value) = front::t_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'f' => {
                if let Some(first_value) = front::f_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b's' => {
                if let Some(first_value) = front::s_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'o' => {
                if let Some(first_value) = front::o_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'e' => {
                if let Some(first_value) = front::e_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'n' => {
                if let Some(first_value) = front::n_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            _ => {},
        };
    }
    unreachable!("ERROR: number not found in line");
}

fn find_last_value(line: &[u8]) -> u32 {
    for (index, character) in line.into_iter().rev().enumerate() {
        let remaining_length: usize = index - 1;
        match character {
            _ if ASCII_RANGE.contains(character) => return (character - ASCII_NUMBER_OFFSET) as u32,
            b't' => {
                if let Some(first_value) = t_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'f' => {
                if let Some(first_value) = f_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b's' => {
                if let Some(first_value) = s_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'o' => {
                if let Some(first_value) = o_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'e' => {
                if let Some(first_value) = e_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            b'n' => {
                if let Some(first_value) = n_number(&line[index + 1..line.len()], remaining_length) {
                    return first_value as u32;
                }
            },
            _ => {},
        };
    }
    unreachable!("ERROR: number not found in line");

}

fn find_line_value(line: &[u8]) -> u32 {
    find_first_value(line);
    dbg!(find_last_value(line));
    return 0;
}

fn handle_line(letter: &u8, index: usize, file_contents: &Vec<u8>, line_start: &mut usize) -> u32 {
    if *letter == NEW_LINE {
        let result: u32 = find_line_value(&file_contents[*line_start..index]);
        *line_start = index + 1;
        return result;
    }
    return 0;
}

pub fn solution(path: &'static str) -> u32 {
    let file_contents: Vec<u8> = std::fs::read(path).expect("ERROR: invalid path");

    let mut line_start: usize = 0;
    file_contents
        .iter()
        .enumerate()
        .fold(0, |accumulator, (index, letter)| accumulator + handle_line(letter, index, &file_contents, &mut line_start))
}
