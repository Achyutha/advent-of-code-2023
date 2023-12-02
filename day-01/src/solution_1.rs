use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn get_first_digit(line: &str) -> usize {
    for char in line.chars() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap() as usize;
        }
    }

    unreachable!()
}

fn get_last_digit(line: &str) -> usize {
    for char in line.chars().rev() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap() as usize;
        }
    }

    unreachable!()
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
