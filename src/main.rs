use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::env;

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

fn find_directory(directories: &Vec<PathBuf>, fragment: &str) -> () {
    match &directories.len() {
        0 => println!(""),
        1 => println!("{:?}", &directories[0]),
        _ => {
            for entry in directories {
                match entry.file_stem() {
                    Some(entry) => {
                        let l_entry = entry.to_ascii_lowercase();
                        match l_entry.to_str() {
                            Some(hit) => {
                                if hit.starts_with(fragment) {
                                    println!("{}", hit);
                                }
                            },
                            None => println!("bad"),
                        }
                    },
                    None => println!("No file stem found in directories"),
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let fragment = &args[2];

    let directories: Vec<PathBuf> = get_directories(path);

    find_directory(&directories, &fragment);
}
