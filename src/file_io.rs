use std::fs;
use std::io::{BufReader, BufWriter, Error};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
const FILE_PATH: &str = "./file/";

fn create_path(filename:&str) -> String{
    format!("{}{}.txt",FILE_PATH, filename)
}

fn create_file(path: &str) -> Result<File, Error> {
    // Create a file
    let file = File::create(path);
    match file {
        Ok(f) => return Ok(f),
        Err(e) => return Err(e),
    }
}

pub fn read_file(filename: &str) -> Result<String, Error> {
    let path = create_path(&filename);

    match fs::read_to_string(&path) {
        Ok(string) => return Ok(string),
        Err(_) => {create_file(&path);
        return Ok(String::from(""))},
    }
    
}

fn open_file(path: &str,) -> File{
    // Open the file in write mode
    OpenOptions::new()
    .read(true)
    .write(true)
    .append(true)
    .create(true)
    .open(path)
    .unwrap()
}

pub fn append_to_end_of_file(filename: &str, line: &str) -> Result<(), Error> {
    let path = create_path(&filename);
    let mut file = open_file(&path);

    if let Err(e) = writeln!(file, "{}", line) {
        return Err(e);
    }

    Ok(())
}

pub fn update_line(filename: &str, index: &usize, new_text: &str) -> Result<(), Error> {
    let path = create_path(&filename);

    let file = open_file(&path);

    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().flatten().collect();

    if index < &lines.len(){
        lines[*index] = new_text.to_string();
    }
    write_file(&path, &lines)?;

    Ok(())

}

pub fn delete_line(filename: &str, index: &usize) -> Result<(), Error> {
    let path = create_path(&filename);

    let file = open_file(&path);
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().flatten().collect();

    if index < &lines.len() {
        lines.remove(*index);
    };
    write_file(&path, &lines)?;

    Ok(())
}

fn write_file(path: &str, content: &Vec<String>) -> Result<(), Error>{
    let mut file = File::create(&path)?;
    for line in content {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    };

    Ok(())
}