

use std::path::{Path,PathBuf};
use walkdir::WalkDir;

use std::process::Command;

pub mod compress;
pub mod io_operations;
pub mod decompress;


pub fn compress_single_file(path: &Path, io: &mut io_operations::ReadWrite, round: bool) {

    io.read_from_file(path.to_string_lossy().to_string()); 

    let mut compr = compress::Compress{
        data: io.data.to_vec(),
        headers: io.headers.to_vec(),
        round: round
    };

    compr.split_and_compress();

    let new_path = path.with_extension("bin");
    io.save_as_binary_file(compr.data, compr.headers, &new_path).expect("Failed to save data to file.");


    let _output = Command::new("xz")
    .arg("-z")
    .arg(new_path) // Specify the file you want to compress
    .output()
    .expect("Failed to execute xz");
}


pub fn decompress_single_file(path: &Path, io: &mut io_operations::ReadWrite) {

    let _output = Command::new("xz")
    .arg("-d")
    .arg(path) // Specify the file you want to compress
    .output()
    .expect("Failed to execute xz");



    let mut decompress = decompress::Decompress{
        data: Vec::new()
    };

    
    let temp_name = path.file_stem()
    .unwrap()
    .to_string_lossy()
    .to_string();

    let mut temp_path = path.to_path_buf();
    temp_path.set_file_name(temp_name);
    temp_path.set_extension("bin");


    io.read_from_binary_file(&temp_path).expect("Failed to read data from the binary file.");

    decompress.data = io.data.to_vec();
    decompress.decode();

    

    let mut new_name = temp_path.file_stem()
    .unwrap()
    .to_string_lossy()
    .to_string();

    new_name.push_str("_decomp");
    let mut new_path = temp_path.to_path_buf();
    new_path.set_file_name(new_name);
    new_path.set_extension("csv");

    let _ = io.save_csv(&new_path, decompress.data);


}


pub fn compress_recursively(path: &Path, io: &mut io_operations::ReadWrite, round: bool){
    for entry in WalkDir::new(path).follow_links(true) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".csv") {
                        let path_buf = PathBuf::from(path).join(file_name);
                        compress_single_file(path_buf.as_path(), io, round);
                    }
                }
            }
        }
    }
}


pub fn decompress_recursively(path: &Path, csv: &mut io_operations::ReadWrite){
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







