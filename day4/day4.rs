use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn does_overlap(hashset1: HashSet<i32>, hashset2: HashSet<i32>) -> bool {
    // Takes two HashSets and determines if one completely
    // contains the other--i.e. if one is a subset of the other.
    hashset1.is_subset(&hashset2) | hashset2.is_subset(&hashset1)
}

// Takes a line from the input file and processes it into two hashsets.
fn to_hashset(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let mut hashset_tuple: (HashSet<i32>, HashSet<i32>) = (HashSet::new(), HashSet::new());

    for (index, range_str) in line.split(',').enumerate() {
        let start_end: Vec<&str> = range_str.trim().split('-').collect();

        if let (Ok(start), Ok(end)) = (start_end[0].parse::<i32>(), start_end[1].parse::<i32>()) {
            for num in start..=end {
                match index {
                    0 => hashset_tuple.0.insert(num),
                    1 => hashset_tuple.1.insert(num),
                    _ => true,
                };
            }
        }
    }

    hashset_tuple
}

fn main() -> Result<(), std::io::Error>{
    let file_path = "adventofcode.com_2022_day_4_input.txt";

    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Do stuff
    let mut answer = 0;
    for line in reader.lines() {
        let line = line?;
        let (hashset1, hashset2) = to_hashset(&line);
        if does_overlap(hashset1, hashset2) {
            answer += 1;
        }
    }

    println!("Number of assignment pairs fully containing the other: {}", answer);

    let mut output_file = File::create("answer.txt")?;
    write!(output_file, "{}", answer)?;
    
    Ok(())
}