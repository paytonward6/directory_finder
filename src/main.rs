use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::env;

// Get all level 1 subdirectories at a given path
fn get_directories(path: &Path) -> Vec<PathBuf> {
    let mut directories: Vec<PathBuf> = Vec::new();
    for entry in path.read_dir().expect("Attempt to read contents of directory has failed!") {
        let directory = entry.as_ref().unwrap().path();
        if directory.is_dir() {
                directories.push(directory);
        }
    }
    directories
}

// Go directly to directory or give prompt to choose directory depending
// on the number of matches
fn goto_directory(directories: &Vec<PathBuf>, fragment: &str) -> () {
    let dir_matches = find_matches(&directories, &fragment);
    match &dir_matches.len() {
        0 => eprintln!("No matches found"), // cd to dir
        1 => println!("{}", &dir_matches[0].to_string()), // cd to only dir
        _ => {
            println!("{}", choose_from_dirs(dir_matches));
        }
    }
}

// Prompt to choose directory since there are multiple matches
fn choose_from_dirs(dir_matches: Vec<String>) -> String {
    eprintln!("Select a directory: ");
    for (pos, item) in dir_matches.iter().enumerate() {
        eprintln!("\t{}: {}", pos + 1, item);
    }
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).expect("Invalid input provided.");
    let result = user_input.trim().parse().unwrap_or(0);
    if 0 < result && result < dir_matches.len() + 1 {
        return dir_matches[result - 1].to_string();
    }
    else {
        return "".to_string();
    }
}

// Find non-case sensitive directory matches of input
fn find_matches(directories: &Vec<PathBuf>, fragment: &str) -> Vec<String>{
    let mut strings: Vec<String> = Vec::new();
    for entry in directories {
        match entry.file_stem() {
            Some(entry) => {
                let l_entry = entry.to_ascii_lowercase();
                match l_entry.to_str() {
                    Some(hit) => {
                        if hit.starts_with(&fragment.to_ascii_lowercase()) {
                            strings.push(String::from(entry.to_str().unwrap()));
                        }
                    },
                    None => eprintln!("Unable to convert match to string."),
                }
            },
            None => eprintln!("No file stem found in directories"),
        }
    }
    strings
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        let path = Path::new(&args[1]);
        let fragment = &args[2];

        let directories: Vec<PathBuf> = get_directories(path);
        goto_directory(&directories, &fragment);
    }
    // else print nothing and `cd` to base directory in .bashrc setup
}
