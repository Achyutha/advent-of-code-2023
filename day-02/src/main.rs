use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod solution_1;
mod solution_2;

pub struct Game {
    pub id: u32,
    pub bags: Vec<Bag>,
}

pub struct Bag {
    pub red: Option<u32>,
    pub blue: Option<u32>,
    pub green: Option<u32>,
}

impl Bag {
    fn new() -> Self {
        Self {
            red: None,
            blue: None,
            green: None,
        }
    }
}

impl From<String> for Game {
    fn from(line: String) -> Self {
        fn parse_id(id_part: String) -> u32 {
            let id = id_part.split(" ").collect::<Vec<_>>()[1];

            id.parse().unwrap()
        }

        fn parse_bags(bag_part: String) -> Vec<Bag> {
            let mut result = vec![];
            let sessions: Vec<_> = bag_part.split("; ").collect();
            for session in sessions {
                let mut bag = Bag::new();
                let groups: Vec<_> = session.split(", ").collect();
                for group in groups {
                    let parsed_group: Vec<_> = group.split(" ").collect();
                    let num = parsed_group[0];
                    let color = parsed_group[1];
                    match color {
                        "red" => bag.red = Some(num.parse().unwrap()),
                        "green" => bag.green = Some(num.parse().unwrap()),
                        "blue" => bag.blue = Some(num.parse().unwrap()),
                        _ => unreachable!(),
                    }
                }
                result.push(bag);
            }

            result
        }

        let line: Vec<_> = line.split(": ").collect();
        let id = parse_id(line[0].to_owned());
        let bags = parse_bags(line[1].to_owned());

        Self { id, bags }
    }
}

fn main() {
    let file_path = PathBuf::from("inputs/question-1.txt");
    match File::open(file_path) {
        Ok(data) => {
            let mut games = vec![];
            for line in BufReader::new(data).lines().filter_map(|line| line.ok()) {
                games.push(Game::from(line));
            }

            let constraint = Game::from("Game 0: 12 red, 13 green, 14 blue".to_owned());

            let result = solution_1::solve(&constraint.bags[0], games);
            println!("The solution for the problem-1 is: {}", result);
        }
        Err(e) => {
            eprintln!("Failed to read the file: {:?}", e);
        }
    };

    let file_path = PathBuf::from("inputs/question-2.txt");
    match File::open(file_path) {
        Ok(data) => {
            let mut games = vec![];
            for line in BufReader::new(data).lines().filter_map(|line| line.ok()) {
                games.push(Game::from(line));
            }

            let result = solution_2::solve(games);
            println!("The solution for the problem-2 is: {}", result);
        }
        Err(e) => {
            eprintln!("Failed to read the file: {:?}", e);
        }
    }
}
