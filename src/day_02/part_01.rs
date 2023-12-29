pub struct Solution;

#[derive(Default, Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_line(line: Vec<&str>) -> Vec<CubeSet> {
    line
        .iter()
        .zip(line.iter().skip(1))
        .into_iter()
        .fold(Vec::<CubeSet>::new(), |mut cube_sets, (previous_word, current_word)| {
            if current_word.ends_with(':') { 
                cube_sets.push(CubeSet::default());
                return cube_sets;
            }

            let current_set: &mut CubeSet = cube_sets.last_mut().unwrap();
            match current_word {
                _ if current_word.starts_with("red") => current_set.red += previous_word.parse::<usize>().unwrap(),
                _ if current_word.starts_with("green") => current_set.green += previous_word.parse::<usize>().unwrap(),
                _ if current_word.starts_with("blue") => current_set.blue += previous_word.parse::<usize>().unwrap(),
                _ => {},
            }
            if current_word.ends_with(';') {
                cube_sets.push(CubeSet::default());
            }
            cube_sets
        })
}

fn parse_input_file(file_path: &'static str) -> Vec<Vec<CubeSet>> {
    std::fs::read_to_string(file_path)
        .expect("ERROR: invalid file path")
        .lines()
        .map(|line: &str| parse_line(line.split_whitespace().collect()))
        .collect()
}

fn sum_possible_games(games: Vec<Vec<CubeSet>>, cube_configuration: CubeSet) -> usize {
    games
        .into_iter()
        .enumerate()
        .fold(0, |sum, (index, cube_sets)| -> usize {
            match cube_sets.into_iter().all(|CubeSet { red, green, blue }| red <= cube_configuration.red && green <= cube_configuration.green && blue <= cube_configuration.blue) {
                true => sum + index + 1,
                false => sum,
            }
        })
}

impl crate::Solution<usize> for Solution {
    fn solution() -> usize {
        let cube_games: Vec<Vec<CubeSet>> = parse_input_file("./src/day_02/input.txt"); 
        let cube_configuration: CubeSet = CubeSet { red: 12, green: 13, blue: 14 };
        return sum_possible_games(cube_games, cube_configuration);
    }
}
