use std::env;
use std::fs;
use std::path::PathBuf;
use scraper::{Html, Selector};
use regex::Regex;

fn main() {
    let args = process_args();
    println!("output location: {}", &args.output_file); //just to shut the compiler up


    let removal_patterns = read_removal_pattern_file(&args.removal_pattern_file);

    let input_paths = fs::read_dir(&args.folder_path).unwrap();

    for path in input_paths {
       process_bookmark_file(&read_bookmark_file(&path.unwrap().path()), &removal_patterns);
    }
}

fn process_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprint!("usage: program <folder path> <output file path> [removal rule file path]");
        std::process::exit(1);
    }

    let mut removal_pattern_path = String::new();

    if args.len() > 2 {
        removal_pattern_path = args[2].clone();
    }

    Arguments{
        folder_path: args[0].clone(),
        output_file: args[1].clone(),
        removal_pattern_file: removal_pattern_path.clone()
    }
}

fn read_removal_pattern_file(path: &String) -> Vec<Regex> {
    //I may actually want this to return a Vec of regex
    if path.is_empty(){
        return Vec::new();
    }

    println!("We are goin to remove stuff");
    //read the file
    let pattern_strings: Vec<String> = match fs::read_to_string(&path) {
        Ok(v) => v.lines().map(String::from).collect(),
        Err(e) => {
            eprintln!("Error reading pattern file: {}", e);
            std::process::exit(1);
        }
    };

    let mut patterns = Vec::new();
    for pattern_string in pattern_strings{
        patterns.push(Regex::new(pattern_string.as_str()).unwrap());
    }

    patterns
}

fn read_bookmark_file(path: &PathBuf) -> String {
  match fs::read_to_string(&path) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("Error reading file: {}:", path.display());
            std::process::exit(1);
        }
   }
}

fn process_bookmark_file(content: &String, removal_patterns: &Vec<Regex>) {

    if removal_patterns.is_empty() != true {
        println!("book marks will be removed")
    }

    let document = Html::parse_document(&content);
    let anchor_seloctor = Selector::parse("a").unwrap();

    for element in document.select(&anchor_seloctor){
        println!("{}", element.value().attr("href").unwrap());

    }
}

struct Arguments {
    folder_path: String,
    output_file: String,
    removal_pattern_file: String
}