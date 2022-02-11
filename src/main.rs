#![feature(str_split_whitespace_as_str)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let original_file_name = "small.txt"; // Path::new("t8.shakespeare.txt");
    let pig_latin_file_name = "output.txt";

    println!("Processing...");
    let read_err = format!("Could not read from {}", original_file_name);
    let write_err = format!("Could not write to {}", pig_latin_file_name);

    let orig_file = File::open(Path::new(original_file_name)).expect(&read_err);
    let mut output_file = File::create(Path::new(pig_latin_file_name)).expect(&write_err);
    let orig_lines_by_line_reader = io::BufReader::new(orig_file).lines();

    for orig_line in orig_lines_by_line_reader {
        let orig_line = orig_line.expect(&read_err);
        let orig_line_words = orig_line.split_whitespace();

        let line = format!("x {}\n", orig_line);
        for orig_word in orig_line_words {
            println!("'{}'", orig_word);
            output_file
                .write(format!("{}\n", orig_word).as_bytes())
                .expect(&write_err);
        }
        output_file
            .write(line.as_bytes())
            .expect("Error writing output.");
    }

    println!("Done.");
}
