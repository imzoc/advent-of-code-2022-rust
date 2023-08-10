use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// Handles the initialization of the stacks in the problem. This is manual work.
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

// Executes the moving of crates in one line of the input file.
fn move_crate(stacks: &mut Vec<Vec<char>>, command: (i32, i32, i32)) {
    for _n in 0..command.0 {
        let c = stacks[command.1 as usize].pop().unwrap();
        stacks[command.2 as usize].push(c);
    }
}

// Returns a command with three components that represent:
// 1. How many crates to move;
// 2. From which stack to move them from;
// 3. And to which stack to move them to.
fn parse_input_line(line: &str) -> (i32, i32, i32) {
    let mut parts: Vec<i32> = Vec::new();
    for word in line.split_whitespace() {
        if let Ok(part) = word.parse::<i32>() {
            parts.push(part);
        }
    }
    
    (parts[0], parts[1] - 1, parts[2] - 1)
}

// Collects the top crate on each stack and returns them as a string.
fn collect_top_crates(stacks: &Vec<Vec<char>>) -> String {
    let mut r = String::new();
    for n in 0..stacks.len() {
        r.push(stacks[n][stacks[n].len() - 1])
    }
    
    r
}

fn main() -> Result<(), std::io::Error> {
    let file_path = "adventofcode.com_2022_day_5_input.txt";
    let mut stacks = create_stacks();

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let starting_line = 11;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line_number + 1 >= starting_line {
            let command = parse_input_line(&line);
            move_crate(&mut stacks, command);         
        }
    }

    let answer = collect_top_crates(&stacks);

    let mut output_file = File::create("answer.txt")?;
    println!("{}", answer);
    writeln!(output_file, "{}", answer)?;

    Ok(())
}