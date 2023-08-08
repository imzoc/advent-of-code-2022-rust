use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn find_misplaced_letters<T>(lines: T) -> Result<Vec<char>, std::io::Error>
where T: Iterator<Item = Result<String, std::io::Error>>,
{
    let mut misplaced_letters: Vec<char> = Vec::new();
    // Iterate over each line in the file 
    for line in lines {
        let line = line?; // Unwrap the Result<_, io::Error>
        
        let first_half_vec: Vec<char> = line.chars().take(line.len() / 2).collect();
        let last_half_vec: Vec<char> = line.chars().skip(line.len() / 2).collect();
        
        let first_half_set: HashSet<char> = first_half_vec.into_iter().collect();
        let last_half_set: HashSet<char> = last_half_vec.into_iter().collect();
        
        let common_chars: Vec<char> = first_half_set.intersection(&last_half_set).cloned().collect();

        misplaced_letters.extend(&common_chars);
    }
    
    Ok(misplaced_letters)
}

// TODO: assign priority values to characters, calculate the priority values of the characters returned by
// find_misplaced_letters, and then add them all up!

fn main() -> io::Result<()> {
    let file_path = "adventofcode.com_2022_day_3_input.txt"; // Replace with the actual file path

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let answer = find_misplaced_letters(reader.lines())?;
    println!("Answer is: {:?}", answer);

    Ok(())
}