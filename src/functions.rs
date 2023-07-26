
use std::{fs::File, fmt::write, io::Write};
use std::path::{Path,PathBuf};
use walkdir::WalkDir;
use std::fs;
use std::io::{Read, Error};
use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::io::{BufRead, BufReader};

pub mod compress;
pub mod csv;
pub mod decompress;


pub fn compress_single_file(path: &Path, mut csv: &mut csv::Csv, round: bool) {

    csv.read_from_file(path.to_string_lossy().to_string()); 

    let mut compr = compress::Compress{
        data: csv.ret_data().to_vec(),
        compressed_data: Vec::new(),
        round: round
    };

    compr.split_and_compress();

    let new_path = path.with_extension("comp");
    save_compressed(&new_path, compr.compressed_data).expect("Failed to save data to file.");


    let output = Command::new("xz")
    .arg("-z")
    .arg(new_path) // Specify the file you want to compress
    .output()
    .expect("Failed to execute xz");
}


pub fn decompress_single_file(path: &Path, mut csv: &mut csv::Csv) {

    let output = Command::new("xz")
    .arg("-d")
    .arg(path) // Specify the file you want to compress
    .output()
    .expect("Failed to execute xz");



    let mut decompress = decompress::Decompress{
        data: Vec::new()
    };

    
    let mut temp_name = path.file_stem()
    .unwrap()
    .to_string_lossy()
    .to_string();

    let mut temp_path = path.to_path_buf();
    temp_path.set_file_name(temp_name);
    temp_path.set_extension("comp");


    csv.read_from_file(temp_path.to_string_lossy().to_string());
    decompress.data = csv.ret_data().to_vec();
    decompress.decode();

    

    let mut new_name = temp_path.file_stem()
    .unwrap()
    .to_string_lossy()
    .to_string();

    new_name.push_str("_decomp");
    let mut new_path = temp_path.to_path_buf();
    new_path.set_file_name(new_name);
    new_path.set_extension("csv");

    save_decompressed(&new_path, decompress.data);




}


pub fn compress_recursively(path: &Path, csv: &mut csv::Csv, round: bool){
    for entry in WalkDir::new(path).follow_links(true) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".csv") {
                        let path_buf = PathBuf::from(path).join(file_name);
                        compress_single_file(path_buf.as_path(), csv, round);
                    }
                }
            }
        }
    }
}


pub fn decompress_recursively(path: &Path, csv: &mut csv::Csv){
    for entry in WalkDir::new(path).follow_links(true) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".comp") {
                        let path_buf = PathBuf::from(path).join(file_name);
                        decompress_single_file(path_buf.as_path(), csv);
                    }
                }
            }
        }
    }
}


fn save_compressed(path: &Path, data:  Vec<Vec<String>>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for row in data {
        let row_str = row.join(",");
        writeln!(file, "{}", row_str)?;
    }
    Ok(())
}


fn save_decompressed(path: &Path, data: Vec<Vec<String>>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for row in data {
        let row_str = row.join(",");
        writeln!(file, "{}", row_str)?;
    }

    Ok(())
}


