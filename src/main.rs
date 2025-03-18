#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unused_must_use)]

use std::error::Error;
use std::fs::{DirEntry, FileType};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use rand::prelude::SliceRandom;

fn main() {
    let base_path = "./src/";
    let completed_path = "./completed.txt";

    let mut rnd = rand::thread_rng();

    if let (Ok(mut source), Ok(completed)) = (get_all_files(base_path), read_completed(completed_path)) {
        source.retain(|path| {
            !completed.contains(path) && "current.rs" != path
        });

        source.shuffle(&mut rnd);
        let completed_tasks = completed.len();
        let left_tasks = source.len();

        println!("total tasks: {}, completed tasks: {}, left tasks: {}", completed_tasks + left_tasks, completed_tasks, left_tasks);
        for path in source {
            println!("{}", path);
        }
    }
}

fn read_completed(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        return Ok(vec![]);
    }

    let reader = BufReader::new(file);
    let mut res  = vec![];
    for line in reader.lines() {
        match line {
            Ok(content) => {
                res.push(content.trim().to_string());
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(e.into());
            },
        }
    }

    Ok(res)
}

fn get_all_files(base_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let dirs = list(base_path, directories)?;

    let mut res = vec![];

    for dir in dirs {
        let path = format!("{}{}", base_path, dir);
        let files = list(&path, files)?;
        for file_name in files {
            if file_name.ends_with(".rs") {
                res.push(file_name);
            }
        }
    }

    Ok(res)
}

fn files(file_type: FileType, entry: DirEntry, res: &mut Vec<String>) {
    if file_type.is_file() {
        if let Ok(name) = entry.file_name().into_string() {
            res.push(name);
        }
    }
}

fn directories(file_type: FileType, entry: DirEntry, res: &mut Vec<String>) {
    if file_type.is_dir() {
        if let Ok(name) = entry.file_name().into_string() {
            res.push(name);
        }
    }
}

fn list<F>(path: &str, f: F) -> Result<Vec<String>, Box<dyn Error>>
where F: Fn(FileType, DirEntry, &mut Vec<String>) -> ()
{
    let mut res = vec![];

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        f(file_type, entry, &mut res);
    }

    Ok(res)
}