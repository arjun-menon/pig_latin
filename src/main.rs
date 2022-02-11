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

    fn transform_to_pig_latin(self) -> String {
        let mut t = self.text;
        if self.alphabetic {
            if let Some(vowel_pos) = find_first_vowel(&t) {
                if vowel_pos == 0 {
                    t.extend_from_slice(&['y', 'a', 'y']);
                } else {
                    let capitalize: bool = t[0].is_uppercase();

                    let mut chars_before_vowel: Vec<_> = t.drain(0..vowel_pos).collect();

                    if capitalize {
                        chars_before_vowel[0] = chars_before_vowel[0].to_ascii_lowercase();
                        t[0] = t[0].to_ascii_uppercase();
                    }

                    t.extend_from_slice(&chars_before_vowel);
                    t.extend_from_slice(&['a', 'y']);
                }
            }
            // Note: if vowel_pos is None, it means `t` has only consonants.
        }
        t.iter().collect()
    }
}

fn find_first_vowel(t: &Vec<char>) -> Option<usize> {
    for (i, c) in t.iter().enumerate() {
        if is_vowel(c) {
            return Some(i);
        }
    }
    None
}

fn is_vowel(c: &char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
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
    let mut prev_alphabetic = false;
    let mut wrap_up = false;

    for c in s.chars() {
        let alphabetic = c.is_ascii_alphabetic();

        if state == State::NotStarted {
            state = if alphabetic {
                State::Alphabetic
            } else {
                State::NotAlphabetic
            };
        } else if state == State::Alphabetic && !alphabetic {
            wrap_up = true;
            state = State::NotAlphabetic;
        } else if state == State::NotAlphabetic && alphabetic {
            wrap_up = true;
            state = State::Alphabetic;
        }

        // wrap up
        if wrap_up {
            tokens.push(Token::new(text, prev_alphabetic));
            text = Vec::new();

            // reset alphabetic
            prev_alphabetic = alphabetic;

            // reset wrap_up
            wrap_up = false;
        }

        // collect char
        text.push(c);
        prev_alphabetic = prev_alphabetic && c.is_alphabetic();
    }
    if text.len() > 0 {
        tokens.push(Token::new(text, prev_alphabetic));
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
            let s: String = tok.transform_to_pig_latin();
            print!("{}", s);
        }
        println!("");

        // output_file
        //     .write(format!("{}\n", output_line).as_bytes())
        //     .expect(&write_err);
    }

    println!("Done.");
}
