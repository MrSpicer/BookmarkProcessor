use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use scraper::{Html, Selector};
use regex::Regex;

fn main() {
    let args = process_args();


    let removal_patterns = read_removal_pattern_file(&args.removal_pattern_file);

    let input_paths: ReadDir = fs::read_dir(&args.folder_path).unwrap();

    for path in input_paths {
        //this is unreadable
       process_bookmark_file(&read_bookmark_file(&path.unwrap().path()), &removal_patterns);
    }

    write_output("", &args.output_file);
}

fn process_args() -> Arguments {

    /**
    Would be nice if this took argument flags
    ex: program -d Directory -o output_file.html -r regex.txt

    loop through args and switch if flag. assign next element value to arg field
    **/

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

//this could probably return an array
fn read_removal_pattern_file(path: &String) -> Vec<Regex> {
    //should this return an array?
    if path.is_empty(){
        return Vec::new();
    }

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
    //todo: this needs a return type
    if removal_patterns.is_empty() != true {
        println!("book marks will be removed") //todo: remove
    }

    let document = Html::parse_document(&content);
    let anchor_seloctor = Selector::parse("a").unwrap();

    for element in document.select(&anchor_seloctor){
        println!("{}", element.value().attr("href").unwrap());
        //todo: check if href matches any regex and remove parent <DT>

    }

    //todo: Select and return the outr <DL>

}

fn write_output(file_content: &str, output_file: &String) -> () {
//stub. signature will probably change
}

struct Arguments {
    folder_path: String,
    output_file: String,
    removal_pattern_file: String
}
