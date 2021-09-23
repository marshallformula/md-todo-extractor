use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct TodoFile {
    pub filename: String,
    pub items: Vec<String>,
}

impl TodoFile {
    pub fn new(filename: String) -> Self {
        TodoFile {
            filename,
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: String) {
        &self.items.push(item);
    }

    pub fn has_todos(&self) -> bool {
        !self.items.is_empty()
    }
}

fn is_md_file(entry: &walkdir::Result<walkdir::DirEntry>) -> bool {
    entry
        .as_ref()
        .map(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn find_md_files<P: AsRef<Path>>(root: P) -> Vec<walkdir::DirEntry> {
    let mut found = vec![];
    for entry in walkdir::WalkDir::new(root)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter(is_md_file)
    {
        match entry {
            Ok(e) => found.push(e),
            Err(e) => eprintln!("Could not process file: {}", e),
        }
    }

    return found;
}

// re: Regex = Regex::new(r"- \[ \] (.*)").unwrap();

pub struct Parser {
    re: Regex,
}

impl Parser {
    pub fn init() -> Parser {
        Parser {
            re: Regex::new(r"- \[ \] (.*)").unwrap(),
        }
    }
    pub fn process_todos(&self, entry: walkdir::DirEntry) -> TodoFile {
        let mut tf = TodoFile::new(
            entry
                .file_name()
                .to_str()
                .map_or(String::from(""), String::from),
        );

        let reg = &self.re;

        if let Ok(lines) = read_lines(entry.path()) {
            for line in lines {
                if let Ok(entry) = line {
                    for cap in reg.captures_iter(&entry) {
                        tf.add_item(cap[1].to_string())
                    }
                }
            }
        }

        return tf;
    }
}

/// Reads the lines from a file into a Buffered Iterator (in a Result)
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}
