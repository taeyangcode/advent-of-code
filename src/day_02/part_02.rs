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

fn power_games(games: Vec<Vec<CubeSet>>) -> usize {
    games
        .into_iter()
        .fold(0, |total, cube_sets| -> usize {
            let minimum_cube_set: CubeSet = cube_sets
                .into_iter()
                .fold(CubeSet::default(), |minimum_set, current_set|
                    CubeSet {
                        red: std::cmp::max(minimum_set.red, current_set.red),
                        green: std::cmp::max(minimum_set.green, current_set.green),
                        blue: std::cmp::max(minimum_set.blue, current_set.blue)
                    }
                );
            total + minimum_cube_set.red * minimum_cube_set.green * minimum_cube_set.blue
        })
}

impl crate::Solution<usize> for Solution {
    fn solution() -> usize {
        let cube_games: Vec<Vec<CubeSet>> = parse_input_file("./src/day_02/input.txt"); 
        return power_games(cube_games);
    }
}

