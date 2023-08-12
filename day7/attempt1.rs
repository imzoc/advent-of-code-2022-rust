use std::fs::File as SysFile;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

/* This code does not work because I can't figure out how to keep and track parent directory
references. */

fn main() -> Result<(), std::io::Error> {
    let file = SysFile::open("adventofcode.com_2022_day_7_input.txt");
    let reader = BufReader::new(file?);
  
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();
    let lines = lines?;
    
    let root_directory = process_input(lines);

    // Find and sum all directories' sizes that are less than 100000.

    /*
    println!("Answer: {:?}", &root_directory);
    let mut output_file = SysFile::create("answer.txt")?;
    writeln!(output_file, "{:?}", root_directory)?;
    */
    Ok(())
}

fn process_input(lines: Vec<String>) -> Directory {
    // Important data
    let mut root_directory = Directory::new("/".to_string());
    let mut working_directory = &mut root_directory;

    for line in lines.into_iter() {
        let line: Vec<&str> = line.split_whitespace().collect();

        if line[0].parse::<i32>().is_ok() { // line starts with number (file)
            working_directory.add_file(line[1].to_string(), line[0].parse::<i32>().unwrap().clone());

        } else if line[0] == "dir" { // line starts with "dir" (subdirectory)
            working_directory.add_subdir(line[1].to_string());

        } else if line[0] == "$" && line[1] == "cd" { // we can ignore "$ ls"
            if line[2] == ".." { // "cd .."
                // change working_directory to parent directory
                
            } else { // "cd subdirectory"
                // change working_directory to subdirectory
                let working_directory = working_directory.move_to_subdir(line[1].to_string());
            }
        }
    }

    root_directory
}

#[derive(Debug)]
pub struct File {
    name: String, // relative path
    size: i32,
}

#[derive(Debug)]
pub struct Directory {
    name: String, // ABSOLUTE path
    files: HashMap<String, File>,
    subdirectories: HashMap<String, Directory>,
}

impl Directory {
    /* Takes an ABOLUTE path name and returns a new, empty Directory with that name. */
    pub fn new(name: String) -> Directory {
        let dir = Directory {
            name: name.to_string(),
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        };

        dir
    }

    /* Takes a filename, creates a File with that name, and inserts it into the Directory's files
    HashMap. */
    pub fn add_file(&mut self, name: String, size: i32) {
        let file = File {
            name: name.to_string(),
            size: size,
        };

        self.files.insert(name.to_string(), file);
    }

    /* Takes a directory's RELATIVE path name, creates a Directory with an absolute path name for
    that subdirectory, and inserts that Directory into the self.subdirectories HashMap. */
    pub fn add_subdir(&mut self, name: String) {
        let name = self.pwd() + &name; // Makes path absolute
        let mut subdir = Directory::new(name.clone());
        
        self.subdirectories.insert(name.to_string(), subdir);
    }

    /*
    /* TODO: Implement a method that allows for traversal to parent directory */
    pub fn add_parent(&mut self, parent: &mut Directory) {
        self.subdirectories.insert("..".to_string(), parent);
    }
    */

    /* Takes a subdirectory's RELATIVE path name and returns a mutable reference to that
    subdirectory (assuming its absolute path name exists in self.subdirectories) */
    pub fn move_to_subdir(&mut self, name: String) -> Option<&mut Directory> {
        let name = self.pwd() + &name; // Makes path absolute

        self.subdirectories.get_mut(&name)
    }

    /* Returns what would be printed in stdout if stdin was "pwd". This is prepended to all
    subdirectories' RELATIVE path names before creating a Directory object for them. */
    pub fn pwd(&mut self) -> String {
        self.name.clone() + "/"
    }
}