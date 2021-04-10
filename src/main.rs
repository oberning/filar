use regex::Regex;
use std::{
    fs, io,
    path::{self, PathBuf},
    usize,
};

const PATH: &str = "C:\\Users\\HP\\Downloads\\TOSEC\\Applications\\[TZX]";
const CHUNK_SIZE: usize = 50;
fn main() -> io::Result<()> {
    let entries = files(PATH)?;
    let regex = Regex::new("^[^[A-Z]]").unwrap();
    let filtered = files_regex(&entries, &regex);
    let mut last_path = "";
    for chunk in filtered.chunks(CHUNK_SIZE) {
        let first_path = chunk
            .first()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        println!("First: {:#?}", first_path);
        println!("Last: {:#?}", last_path);
        last_path = chunk.last().unwrap().file_name().unwrap().to_str().unwrap();
    }
    //println!("Ergebnis: {:#?}, Size: {}", test, test.len());
    Ok(())
}

fn files(path: &str) -> Result<Vec<path::PathBuf>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .filter(|entry| entry.as_ref().unwrap().path().is_file())
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));
    Ok(entries)
}

fn files_regex(v: &Vec<PathBuf>, regex: &Regex) -> Vec<PathBuf> {
    v.to_owned()
        .into_iter()
        .filter(|entry| regex.is_match(entry.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<_>>()
}