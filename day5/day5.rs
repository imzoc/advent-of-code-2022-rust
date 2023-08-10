use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn create_stacks() -> Vec<Vec<char>> {
    let str_stacks:Vec<String> = vec![
        "NSDCVQT".to_string(),
        "MFV".to_string(),
        "FQWDPNHM".to_string(),
        "DQRTF".to_string(),
        "RFMNQHVB".to_string(),
        "CFGNPWQ".to_string(),
        "WFRLCT".to_string(),
        "TZNS".to_string(),
        "MSDJRQHN".to_string()
    ];

    let mut char_stacks: Vec<Vec<char>> = Vec::new();
    for str_stack in str_stacks {
        char_stacks.push(str_stack.chars().collect());
    }

    char_stacks
}

fn move_crate(stacks: Vec<Vec<char>>, command: (i32, i32, i32)) -> Vec<Vec<char>> {
    for n in 0..=command.0 {

    }


    stacks
}

fn parse_input_line(line: &str) -> (i32, i32, i32) {
    let mut parts: Vec<i32> = Vec::new();
    for word in line.split_whitespace() {
        if let Ok(part) = word.parse::<i32>() {
            parts.push(part);
        }
    }
    
    (parts[0], parts[1], parts[2])
}

fn main() -> Result<(), std::io::Error> {
    let file_path = "adventofcode.com_2022_day_5_input.txt";
    let stacks = create_stacks();
    for row in &stacks {
        println!("{:?}", row);
    }

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let starting_line = 12;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line_number + 1 >= starting_line {
            let command = parse_input_line(&line);
            let stacks = move_crate(stacks.clone(), command);         
        }
    }

    // TODO: implement moving function and take the last crate from each stack as the answer.
     
    let mut output_file = File::create("answer.txt")?;
    for row in &stacks {
        println!("{:?}", row);
        writeln!(output_file, "{:?}", row)?;
    }

    Ok(())
}