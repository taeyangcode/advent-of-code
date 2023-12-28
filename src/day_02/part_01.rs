use std::ops::RangeInclusive;

#[derive(Default, Debug)]
struct GameSet {
    red: usize,
    blue: usize,
    green: usize,
}

fn line_result(line: &[u8]) -> u32 {
    todo!();
}

fn cube_id_sum(file_contents: String) -> u32 {
    let mut game_data: Vec<[GameSet; 3]> = vec![];

    const ASCII_NUMBER_RANGE: RangeInclusive<u8> = 47..=57;

    let mut start_index: usize = 0;
    let mut game_set_index: usize = 0;
    for (index, current_character) in file_contents.as_bytes().iter().enumerate() {
        match current_character {
            _ if ASCII_NUMBER_RANGE.contains(current_character) => if start_index != 0 { start_index = index; },
            // b'r' => game_data.last().unwrap()[game_set_index].red = file_contents[start_index..(index - 1)].parse().unwrap(),
            // b'g' => game_data.last().unwrap()[game_set_index].green = file_contents[start_index..(index - 1)].parse().unwrap(),
            // b'b' => game_data.last().unwrap()[game_set_index].blue = file_contents[start_index..(index - 1)].parse().unwrap(),
            b':' => {
                game_data.push([GameSet::default(), GameSet::default(), GameSet::default()]);
                start_index = 0;
                game_set_index = 0;
            },
            b';' => game_set_index += 1,
            _ => {},
        }
    }

    println!("{:?}", game_data);
    return 0;
}

pub fn solution(file_path: &'static str) -> u32 {
    let input_contents: String = std::fs::read_to_string(file_path).expect("ERROR: opening input file");
    return cube_id_sum(input_contents);
}
