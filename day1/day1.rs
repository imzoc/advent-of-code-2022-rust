use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let file_path = "adventofcode.com_2022_day_1_input.txt"; // Replace with the actual file path

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Data
    let mut max_calories = 0;
    let mut current_calories = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the Result<_, io::Error>

        // Check if the line is blank (contains only whitespace or is empty)
        if line.trim().is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            if let Ok(number) = line.trim().parse::<i32>() {
                current_calories += number;
            }
        }
    }

    println!("Sum = {}", max_calories);

    let mut output_file = File::create("answer.txt")?;
    write!(output_file, "{}", max_calories)?;

    Ok(())
}
