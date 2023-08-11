use std::fs::File;
use std::io::{BufRead, BufReader, Write};


// 
fn has_duplicates(frame: &[char]) -> bool {
    for (i, &c1) in frame.into_iter().enumerate() {
        for &c2 in frame.into_iter().skip(i+1) {
            if c1 == c2 {
                println!("Frame: {}; has_duplicates: {:?}.", print_frame(frame), true);
                return true;
            }
        }
    }
    println!("Frame: {}; has_duplicates: {:?}.", print_frame(frame), false);
    false
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("adventofcode.com_2022_day_6_input.txt");
    let reader = BufReader::new(file?);
    
    let all: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();
    let mut frame = &all[0..=3];

    let mut n = 0;
    loop {
        if has_duplicates(frame) {
            n += 1;
            frame = &all[n..n+4];
        } else {
            break;
        }
    }

    n += 4;

    println!("Answer: {}", n);
    let mut output_file = File::create("answer.txt")?;
    writeln!(output_file, "{}", n)?;

    Ok(())
}

fn print_frame(frame: &[char]) -> String {
    let r: String = frame.iter().collect();
    r
}