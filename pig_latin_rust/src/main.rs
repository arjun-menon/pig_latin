#![feature(let_chains)]
#![allow(non_snake_case)]
#![feature(str_split_whitespace_as_str)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Processing...");

    let originalFilePath = Path::new("small.txt"); // Path::new("t8.shakespeare.txt");
    let pigLatinFilePath = Path::new("output.txt");

    if let Ok(originalFile) = File::open(originalFilePath) &&
       let Ok(mut pigLatinFile) = File::create(pigLatinFilePath) {
        let origLines = io::BufReader::new(originalFile).lines();
        for origLineR in origLines {
            if let Ok(origLine) = origLineR {
                let origLineWords = origLine.split_whitespace();
                let line = format!("x {}\n", origLine);
                for origWord in origLineWords {
                    println!("'{}'", origWord);
                    pigLatinFile.write(format!("{}\n", origWord).as_bytes());
                }
                pigLatinFile.write(line.as_bytes()).expect("Error writing output.");   
            }
        }
    }

    println!("Done.");
}
