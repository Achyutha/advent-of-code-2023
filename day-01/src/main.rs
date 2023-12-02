use std::path::PathBuf;

mod solution_1;
mod solution_2;

fn main() {
    let file_path = PathBuf::from("inputs/question-1.txt");
    let solution_1 = solution_1::solve(file_path);

    println!("The solution for the first question is: {}", solution_1);

    let file_path = PathBuf::from("inputs/question-2.txt");
    let solution_2 = solution_2::solve(file_path);

    println!("The solution for the second question is: {}", solution_2);
}
