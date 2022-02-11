#![feature(str_split_whitespace_as_str)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

struct Token {
    text: Vec<char>,
    alphabetic: bool,
}
impl Token {
    fn new(text: Vec<char>, alphabetic: bool) -> Token {
        assert!(text.len() > 0);
        Token { text, alphabetic }
    }
}

fn transform(tok: Token) -> String {
    let t = tok.text;
    if tok.alphabetic {
        let s: String = "".to_string();

        let is_capitalized: bool = t[0].is_uppercase();

        t.iter().collect()
    } else {
        t.iter().collect()
    }
}

fn str_to_tokens(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    #[derive(PartialEq, Eq)]
    enum State {
        NotStarted,
        Alphabetic,
        NotAlphabetic,
    }
    let mut state = State::NotStarted;

    let mut text: Vec<char> = Vec::new();
    let mut alphabetic: bool = false;
    let mut wrap_up: bool = false;

    for c in s.chars() {
        if state == State::NotStarted {
            state = if c.is_alphabetic() {
                State::Alphabetic
            } else {
                State::NotAlphabetic
            };
        } else if state == State::Alphabetic && !c.is_alphabetic() {
            wrap_up = true;
            state = State::NotAlphabetic;
        } else if state == State::NotAlphabetic && c.is_alphabetic() {
            wrap_up = true;
            state = State::Alphabetic;
        }

        // wrap up
        if wrap_up {
            tokens.push(Token::new(text, alphabetic));
            text = Vec::new();

            // reset alphabetic
            alphabetic = c.is_alphabetic();

            // reset wrap_up
            wrap_up = false;
        }

        // collect char
        text.push(c);
        alphabetic = alphabetic && c.is_alphabetic();
    }
    if text.len() > 0 {
        tokens.push(Token::new(text, alphabetic));
    }

    tokens
}

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

        for tok in str_to_tokens(&orig_line) {
            let s: String = transform(tok);
            print!("{}", s);
        }
        println!("");

        // output_file
        //     .write(format!("{}\n", output_line).as_bytes())
        //     .expect(&write_err);
    }

    println!("Done.");
}
