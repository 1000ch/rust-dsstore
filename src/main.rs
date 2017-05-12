extern crate glob;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use glob::glob;

fn print_usage(program: &str) {
    println!("Usage: {} <path>", program);
}

fn get_targets(args: &Vec<String>) -> glob::Paths {
    let glob_path = if args.len() == 2 {
        PathBuf::from(format!("{}/**/.DS_Store", args[1]))
    } else {
        match env::current_dir() {
            Ok(cwd) => cwd.join("**/.DS_Store"),
            Err(_) => process::exit(0x0f00)
        }
    };

    let glob_str = match glob_path.to_str() {
        Some(str) => str,
        None => process::exit(0x0f00)
    };

    glob(&glob_str).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() >= 3 {
        print_usage(&program);
        process::exit(0x0f00);
    }

    let targets = get_targets(&args);

    for target in targets.filter_map(Result::ok) {
        match fs::remove_file(target.as_path()) {
            Ok(_) => println!("Removed {:?}", target.display()),
            Err(e) => println!("Could not remove {}: {:?}", target.display(), e.to_string())
        }
    }
}
