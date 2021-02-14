mod wordlist;

extern crate getopts;
use getopts::Options;
use rand::prelude::*;
use std::io::{BufRead, BufReader};
use std::string::ToString;
use std::fs::File;
use std::env;

use wordlist::default_word_list;

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_random_words<T: ToString>(word_list: &Vec<T>, num_words: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..num_words).map(|_| word_list[rng.gen_range(0..word_list.len())].to_string()).collect()
}

fn make_passphrase(input_filename: String, separator: String, num_words: usize, capitalize: usize, max_num: i32) {
    let mut word_list: Vec<String> = Vec::new();
    if input_filename != "" {
        let reader = BufReader::new(File::open(input_filename).expect("Cannot open file."));
        word_list = reader.lines().map(|line| line.unwrap()).collect();
    };
    let mut phrase_words: Vec<String>;
    if word_list.len() > 0 {
        phrase_words = get_random_words(&word_list, num_words);
    } else {
        phrase_words = get_random_words(&default_word_list(), num_words);
    }

    if capitalize > 0 && capitalize < num_words {
        phrase_words[capitalize - 1] = some_kind_of_uppercase_first_letter(&phrase_words[capitalize - 1]);
    }

    let mut rng = rand::thread_rng();
    let extra_number = if max_num > 0 {rng.gen_range(0..=max_num).to_string()} else {"".to_string()};
    println!("{}{}", phrase_words.join(&separator), extra_number);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // get command-line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("w", "words", "number of words to pick", "4");
    opts.optopt("s", "separator", "separator between words", "-");
    opts.optopt("c", "capitalize", "word number to capitalize", "1");
    opts.optopt("f", "filename", "word file to choose from", "FILENAME (defaults to internal wordlist)");
    opts.optopt("n", "number", "maximum number to randomly append", "9");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // set options/defaults based on arguments
    let input_filename = matches.opt_get_default("f", String::from("")).unwrap();
    let separator = matches.opt_get_default("s", String::from("-")).unwrap();
    let num_words: usize = matches.opt_get_default("w", 4).unwrap();
    let capitalize: usize = matches.opt_get_default("c", 1).unwrap();
    let max_num = matches.opt_get_default("n", 9).unwrap();
    
    // call the actual program
    make_passphrase(input_filename, separator, num_words, capitalize, max_num);
}