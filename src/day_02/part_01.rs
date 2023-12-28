use std::ops::RangeInclusive;

#[derive(Default, Debug)]
struct GameSet {
    red: usize,
    blue: usize,
    green: usize,
}

fn parse_file_contents(file_contents: String) -> Vec<Vec<GameSet>> {
    const ASCII_NUMBER_RANGE: RangeInclusive<u8> = 47..=57;

    let mut game_data: Vec<Vec<GameSet>> = vec![];
    let mut start_index: usize = 0;
    for (index, current_character) in file_contents.as_bytes().iter().enumerate() {
        match current_character {
            _ if ASCII_NUMBER_RANGE.contains(current_character) => if start_index == 0 { start_index = index; },
            b'r' => {
                if file_contents.as_bytes()[index - 1] != b' ' { continue; }
                game_data.last_mut().unwrap().last_mut().unwrap().red += file_contents[start_index..(index - 1)].parse::<usize>().unwrap();
                start_index = 0;
            }
            b'g' => {
                if file_contents.as_bytes()[index - 1] != b' ' { continue; }
                game_data.last_mut().unwrap().last_mut().unwrap().green += file_contents[start_index..(index - 1)].parse::<usize>().unwrap();
                start_index = 0;
            }
            b'b' => {
                if file_contents.as_bytes()[index - 1] != b' ' { continue; }
                game_data.last_mut().unwrap().last_mut().unwrap().blue += file_contents[start_index..(index - 1)].parse::<usize>().unwrap();
                start_index = 0;
            }
            b':' => {
                game_data.push(Vec::from([GameSet::default()]));
                start_index = 0;
            },
            b';' => game_data.last_mut().unwrap().push(GameSet::default()),
            _ => {},
        }
    }

    return game_data;
}

fn valid_id_sum(game_data: Vec<Vec<GameSet>>, (red_max, green_max, blue_max): (usize, usize, usize)) -> usize {
    game_data
        .into_iter()
        .enumerate()
        .fold(0, |accumulator, (index, gamesets)| {
            if gamesets.iter().all(|GameSet { red, blue, green}| red <= &red_max && green <= &green_max && blue <= &blue_max) {
                return accumulator + index + 1;
            }
            accumulator
        })
}

pub fn solution(file_path: &'static str) -> u32 {
    let input_contents: String = std::fs::read_to_string(file_path).expect("ERROR: opening input file");
    return valid_id_sum(parse_file_contents(input_contents), (12, 13, 14)) as u32;
}
