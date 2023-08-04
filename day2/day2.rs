use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let file_path = "adventofcode.com_2022_day_2_input.txt"; // Replace with the actual file path

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Data
    let mut total_score = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the Result<_, io::Error>

        let characters: Vec<char> = line.chars().collect();

        if characters[2] == 'X' {
            total_score += 1;
            
            if characters[0] == 'A' {
                // Draw.
                total_score += 3;

            } else if characters[0] == 'B' {
                // Lose.


            } else if characters[0] == 'C' {
                // Win.
                total_score += 6;

            } else {
                println!("Unexpected Character.")
            }
        } else if characters[2] == 'Y' {
            total_score += 2;

            if characters[0] == 'A' {
                // Win.
                total_score += 6;

            } else if characters[0] == 'B' {
                // Draw.
                total_score += 3;

            } else if characters[0] == 'C' {
                // Lose.


            } else {
                println!("Unexpected character.")
            }
        } else if characters[2] == 'Z' {
            total_score += 3;

            if characters[0] == 'A' {
                // Lose.


            } else if characters[0] == 'B' {
                // Win.
                total_score += 6;

            } else if characters[0] == 'C' {
                // Draw.
                total_score += 3;

            } else {
                println!("Unexpected character.")
            }
        } else {
            println!("Unexpected character.")
        }
    }

    println!("Round score is {}", total_score);

    let mut output_file = File::create("answer.txt")?;
    write!(output_file, "{}", total_score)?;

    Ok(())
}