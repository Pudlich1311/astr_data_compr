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
            println!("  [-cr file] compress a specific file with rounding");
            println!("  [-c file] compress a specific file");
            println!("  [-d file] decompress a specific file");
            println!("  [-recc path] compress recursively all csv files in path");
            println!("  [-reccr path] compress recursively all csv files in path with rounding");
            println!("  [-recd path] decompress recursively all compressed files in path");
        }

        if *argument == "-cr"{
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_single_file(path, &mut csv,true)
            } else {
                println!("Error");
            }
            break;
        }
        if *argument == "-c" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_single_file(path, &mut csv,false)
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

        if *argument == "-recc" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_recursively(path, &mut csv,false);
            } else {
                println!("Error");
            }
            break;
        }

        if *argument == "-reccr" {
            if let Some(next_argument) = iter.next() {
                let path: &Path = Path::new(next_argument.1);
                functions::compress_recursively(path, &mut csv,true);
            } else {
                println!("Error");
            }
            break;
        }

        if *argument == "-recd" {
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

