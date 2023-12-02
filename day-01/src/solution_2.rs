use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const NUMS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

const REVERSE_NUMS: &[&str] = &[
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn word_to_digit(word: &str) -> usize {
    match word {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => word.parse().unwrap(),
    }
}

fn get_first_digit(line: &str) -> usize {
    let mut ans_str = "";
    let mut ans_pos = usize::MAX;

    for digit in NUMS.into_iter() {
        let found_val = line.find(digit);
        if found_val.is_some() && found_val.unwrap() < ans_pos {
            ans_str = digit;
            ans_pos = found_val.unwrap();
        }
    }

    word_to_digit(ans_str)
}

fn get_last_digit(line: &str) -> usize {
    let mut ans_str = "";
    let mut ans_pos = usize::MAX;
    let line: &str = &line.chars().rev().collect::<String>()[..];
    for digit in REVERSE_NUMS.into_iter() {
        let found_val = line.find(digit);
        if found_val.is_some() && found_val.unwrap() < ans_pos {
            ans_str = digit;
            ans_pos = found_val.unwrap();
        }
    }

    word_to_digit(ans_str)
}

pub fn solve(filename: PathBuf) -> usize {
    let mut ans = 0;
    match File::open(filename) {
        Ok(data) => {
            let data = BufReader::new(data).lines().filter_map(|line| line.ok());
            for line in data {
                let first_digit = get_first_digit(&line);
                let last_digit = get_last_digit(&line);

                ans += first_digit * 10 + last_digit;
            }
        }
        Err(e) => {
            eprintln!("File provided could not be opened: {:?}", e)
        }
    };

    ans
}
