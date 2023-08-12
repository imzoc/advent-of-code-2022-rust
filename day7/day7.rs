use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

// PSEUDOCODE
// We have a filetree. We need to find all directories with up to 100000 size.
// The input we are given starts with '$ cd /\n$ ls'. It outputs the contents of
// '/' up until the next '$', which then changes directory.

fn main() -> Result<(), std::io::Error> {
    let file = File::open("adventofcode.com_2022_day_7_input.txt");
    let reader = BufReader::new(file?);

    let lines: Vec<String> = reader.lines().collect();

    // Important variables
    let mut good_dirs: Vec<String> = Vec::new();
    let mut total_size = 0;

    // We have a mixture of commands and stdout.
    let mut current_directory = String::new();
    let mut prev_directories: Vec<String> = Vec::new();
    let mut contents = 
    for line in lines.into_iter() {
        let line = line?;
        let line = line.split_whitespace().collect();
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == ".." {
                    let current_directory = prev_directories.pop()?;
                }
                let current_directory = line[2];
                // Save contents somehow
            } // The only other possibility is 'ls', and we do nothing because
            // it always comes after 'cd'.
            
        }



    println!("Answer: {}", total_size);
    let mut output_file = File::create("answer.txt")?;
    writeln!(output_file, "{}", n)?;

    Ok(())
}


// I think using structs will help me. I need to figure out how to use them.
// Yep! ChatGPT put me on the right path :)

#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    files: HashMap<String, File>,
    subdirectories: HashMap<String, Directory>,
}

impl Directory {
    pub fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: &str, size: u32) {
        let file = File {
            name: name.to_string(),
            size: size,
        };

        self.files.insert(name.to_string(), file);
    }

    pub fn add_subdir(&mut self, name: &str) -> &mut Directory {
        let subdir = Directory::new(name);
        self.subdirectories.insert(name.to_string(), subdir);
        
        return self.subdirectories.get_mut(name).unwrap()
    }
}