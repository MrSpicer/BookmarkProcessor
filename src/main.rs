use std::env;

fn main() {
    let args = process_args();
    println!("input folder: {}",args.folder_path);
    println!("output file path: {}", args.output_file);
}

fn process_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        eprint!("usage: program <folderpath> <outputfile>");
        std::process::exit(1);
    }

    Arguments{
        folder_path: args[0].clone(),
        output_file: args[1].clone()
    }
}

struct Arguments {
    folder_path: String,
    output_file: String
}