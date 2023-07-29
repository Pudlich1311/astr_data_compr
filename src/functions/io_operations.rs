use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct ReadWrite{
    pub data: Vec<Vec<String>>,
    pub headers: Vec<String>
}


impl ReadWrite{

    pub fn read_from_file(&mut self, path: String){

        self.data.clear();
        self.headers.clear();
        let mut rdr = csv::Reader::from_path(path).unwrap();
        for header in rdr.headers().unwrap().iter(){
            self.headers.push(header.to_string());
        }


        for row in rdr.records() {
            let mut temp = Vec::new();
            for r in row.unwrap().iter() {
                temp.push(r.to_string());
            }
            self.data.push(temp);
        }
    
    }

    pub fn save_csv(&self,path: &Path, data:  Vec<Vec<String>>) -> std::io::Result<()> {
        let mut file = File::create(path)?;
    
    
        for row in data {
            let row_str = row.join(",");
            writeln!(file, "{}", row_str)?;
        }
        Ok(())
    }

    pub fn read_from_binary_file(& mut self, path: &Path) -> std::io::Result<Vec<Vec<String>>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        // Split the buffer into fields using null characters as separators
        let mut data = Vec::new();
        let mut row: Vec<String> = Vec::new();
        let mut field = String::new();
    
        for byte in buffer {
            if byte == 0 {
                row.push(field.clone());
                field.clear();
            } else {
                field.push(byte as char);
            }
        }
    
        data.push(row);
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
    
        for row in data.iter() {
            for field in row.iter() {
                if field.contains('\n') {
                    let mut parts = field.split('\n').peekable();
                    while let Some(part) = parts.next() {
                        current_row.push(part.to_string());
                        if parts.peek().is_some() {
                            rows.push(current_row);
                            current_row = Vec::new();
                        }
                    }
                } else {
                    current_row.push(field.to_string());
                }
            }
    
            rows.push(current_row);
            current_row = Vec::new();
        }

        self.data=rows.to_vec();
        Ok(data)
    }

    pub fn save_as_binary_file(&self, data: Vec<Vec<String>>, headers: Vec<String>, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
    
        let mut first = true;
        for field in headers {
            if !first {
                // Add a null character as a separator between fields
                file.write_all(&[0])?;
            }
            first = false;
            file.write_all(field.as_bytes())?;
        }
        file.write_all(&[b'\n'])?;
    
        for row in data {
            let mut first = true;
            for field in row {
                if !first {
                    // Add a null character as a separator between fields
                    file.write_all(&[0])?;
                }
                first = false;
                file.write_all(field.as_bytes())?;
            }
    
            // Add a newline character as a separator between rows
            file.write_all(&[b'\n'])?;
        }
    
        Ok(())
    }


}