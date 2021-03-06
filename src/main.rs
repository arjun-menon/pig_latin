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
        let mut text = self.text;
        if self.alphabetic {
            if let Some(vowel_pos) = find_first_vowel(&text) {
                if vowel_pos == 0 {
                    text.extend_from_slice(&['y', 'a', 'y']);
                } else {
                    let capitalized: bool = text[0].is_uppercase();

                    let mut chars_before_vowel: Vec<_> = text.drain(0..vowel_pos).collect();

                    if capitalized {
                        chars_before_vowel[0] = chars_before_vowel[0].to_ascii_lowercase();
                        text[0] = text[0].to_ascii_uppercase();
                    }

                    text.extend_from_slice(&chars_before_vowel);
                    text.extend_from_slice(&['a', 'y']);
                }
            }
            // Note: if vowel_pos is None, it means `t` has only consonants.
        }
        text.into_iter().collect()
    }
}

fn find_first_vowel(t: &Vec<char>) -> Option<usize> {
    for (i, c) in t.iter().enumerate() {
        if is_vowel(*c) {
            return Some(i);
        }
    }
    None
}

fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn str_to_tokens(s: String, add_newline: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    if s.len() == 0 {
        return tokens;
    }

    let mut text: Vec<char> = Vec::new();
    let mut alphabetic = first_char(&s).is_ascii_alphabetic();

    for ch in s.chars() {
        if alphabetic != ch.is_ascii_alphabetic() {
            // wrap up if there is a transition
            tokens.push(Token::new(text, alphabetic));
            text = Vec::new(); // reset text
            alphabetic = !alphabetic; // reset alphabetic
        }

        text.push(ch); // collect char
    }

    tokens.push(Token::new(text, alphabetic)); // text will alway be non-empty, i.e. text.len() > 0

    if add_newline {
        tokens.push(Token::new(vec!['\r', '\n'], false))
    }

    tokens
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
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

    /// Use less RAM (random access memory)
    #[clap(short, long)]
    lowmem: bool,
}

fn main() {
    let args = Args::parse();

    let original_file_name: String = match args.filename {
        None => {
            let default_input = "t8.shakespeare.txt";
            println!("Defaulting to {}", default_input);
            default_input.to_string()
        }
        Some(_) => args.filename.unwrap(),
    };
    let pig_latin_file_name = args.output;

    let read_err = format!("Could not read from {}", original_file_name);
    let write_err = format!("Could not write to {}", pig_latin_file_name);

    let mut output_file = File::create(&pig_latin_file_name).expect(&write_err);

    let lowmem = " with lower memory usage";
    println!("Processing{}...", if args.lowmem { lowmem } else { "" });

    if !args.lowmem {
        output_file
            .write(
                str_to_tokens(
                    fs::read_to_string(&original_file_name).expect(&read_err),
                    false,
                )
                .into_par_iter() // process tokens in paralell
                .map(|tok| tok.transform_to_pig_latin())
                .reduce(|| "".to_string(), |cur: String, nxt: String| cur + &nxt)
                .as_bytes(),
            )
            .expect(&write_err);
    } else {
        // when run with --lowmem, use less memory by processing in chunks
        io::BufReader::new(File::open(&original_file_name).expect(&read_err))
            .lines() // read the file line-by-line
            .flat_map(|line| str_to_tokens(line.expect(&read_err), true))
            .map(|tok| tok.transform_to_pig_latin())
            .for_each(|line| {
                output_file.write(line.as_bytes()).expect(&write_err);
            });
    }

    println!("Done.");
}
