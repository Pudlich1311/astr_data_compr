#![allow(dead_code)]
#![allow(unused)]
mod functions;
use std::env;
use crate::functions::csv;
use std::path::Path;

fn main() {
    let mut csv = csv::Csv::default();

    let args: Vec<String> = env::args().collect();
    
    let mut iter = args.iter().skip(1).enumerate();

    while let Some((index, argument)) = iter.next() {
        if *argument == "-h" {
            println!("Help:");
            println!("  [-c file] compress a specific file");
            println!("  [-d file] decompress a specific file");
            println!("  [-rc path] compress recursively all csv files in path");
            println!("  [-rd path] decompress recursively all compressed files in path");
        }

        if *argument == "-c" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_single_file(path, &mut csv)
            } else {
                println!("Error");
            }
            break;
        }

        if *argument == "-d" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::decompress_single_file(path, &mut csv)
            } else {
                println!("Error");
            }
            break;
        }

        if *argument == "-rc" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_recursively(path, &mut csv);
            } else {
                println!("Error");
            }
            break;
        }

        if *argument == "-rd" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::decompress_recursively(path, &mut csv);
            } else {
                println!("Error");
            }
            break;
        }
    }
    
}

