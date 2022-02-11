use clap::Parser;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufRead};

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
                    let capitalized: bool = t[0].is_uppercase();

                    let mut chars_before_vowel: Vec<_> = t.drain(0..vowel_pos).collect();

                    if capitalized {
                        chars_before_vowel[0] = chars_before_vowel[0].to_ascii_lowercase();
                        t[0] = t[0].to_ascii_uppercase();
                    }

                    t.extend_from_slice(&chars_before_vowel);
                    t.extend_from_slice(&['a', 'y']);
                }
            }
            // Note: if vowel_pos is None, it means `t` has only consonants.
        }
        t.into_iter().collect()
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

fn str_to_tokens(s: &str, add_newline: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut text: Vec<char> = Vec::new();
    let mut alphabetic: Option<bool> = None;
    let mut wrap_up = false;

    for c in s.chars() {
        let c_alphabetic = c.is_ascii_alphabetic();

        match alphabetic {
            None => {
                alphabetic = Some(c_alphabetic);
            }
            Some(alphabetic) => {
                wrap_up = alphabetic != c_alphabetic;
            }
        }

        if wrap_up {
            tokens.push(Token::new(text, alphabetic.unwrap()));
            text = Vec::new();

            // reset alphabetic
            alphabetic = Some(c_alphabetic);

            // reset wrap_up
            wrap_up = false;
        }

        // collect char
        text.push(c);

        alphabetic = Some(alphabetic.unwrap() && c_alphabetic);
    }
    if text.len() > 0 {
        tokens.push(Token::new(text, alphabetic.unwrap()));
    }

    if add_newline {
        tokens.push(Token::new(vec!['\r', '\n'], false))
    }

    tokens
}

/// Transform text into pig latin
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    /// File name to process
    filename: Option<String>,

    /// Output file name
    #[clap(short, long, default_value_t = String::from("output.txt"))]
    output: String,

    /// Process faster
    #[clap(short, long)]
    fast: bool,
}

fn main() {
    let args = Args::parse();

    let original_file_name: String = match args.filename {
        None => {
            let default_input = "t8.shakespeare.txt"; // "small.txt"
            println!("Defaulting to {}", default_input);
            default_input.to_string()
        }
        Some(_) => args.filename.unwrap(),
    };
    let pig_latin_file_name = args.output;

    let read_err = format!("Could not read from {}", original_file_name);
    let write_err = format!("Could not write to {}", pig_latin_file_name);

    let mut output_file = File::create(&pig_latin_file_name).expect(&write_err);

    println!(
        "Processing{}...",
        if !args.fast { "" } else { " with fast enabled" }
    );

    if args.fast {
        output_file
            .write(
                str_to_tokens(
                    &fs::read_to_string(&original_file_name).expect(&read_err),
                    false,
                )
                .into_par_iter() // process tokens in paralell
                .map(|tok| tok.transform_to_pig_latin())
                .reduce(|| "".to_string(), |cur: String, nxt: String| cur + &nxt)
                .as_bytes(),
            )
            .expect(&write_err);
    } else {
        // when run without --fast, use less memory by processing in chunks
        io::BufReader::new(File::open(&original_file_name).expect(&read_err))
            .lines() // read the file line-by-line
            .flat_map(|line| str_to_tokens(&line.expect(&read_err), true))
            .map(|tok| tok.transform_to_pig_latin())
            .for_each(|line| {
                output_file.write(line.as_bytes()).expect(&write_err);
            });
    }

    println!("Done.");
}
