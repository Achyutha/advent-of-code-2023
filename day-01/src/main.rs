use std::path::PathBuf;

mod solution_1;
fn main() {
    let file_path = PathBuf::from("inputs/question-1.txt");
    let solution_1 = solution_1::solve(file_path);

    println!("The solution is: {}", solution_1);
}
