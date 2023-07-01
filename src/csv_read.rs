use std::{fs::File, fmt::write, io::Write};


pub struct Csv{
    data: Vec<Vec<String>>
}

impl Default for Csv {
    fn default () -> Csv {
        Csv { data: Vec::new() }
    }
}

impl Csv{
    

    pub fn read(&mut self){
        let mut rdr = csv::Reader::from_path("Gaiatest.csv").unwrap();
        
        let mut headers = Vec::new();
        for header in rdr.headers().unwrap().iter(){
            headers.push(header.to_string());
        }

        self.data.push(headers);

        for row in rdr.records() {
            let mut temp = Vec::new();
            for r in row.unwrap().iter() {
                temp.push(r.to_string());
            }
            self.data.push(temp);
        }
    }


    pub fn save(&self, modified_data: Vec<Vec<String>>){

        let mut file = match File::create("output.csv") {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to create file: {}", e);
                return;
            }
        };

        for val in modified_data.iter(){
            let line = val.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(",");
            write!(file,"{}\n",line);
        }

    }

    pub fn ret_data(&self) -> &Vec<Vec<String>>{
        return &self.data;
    }


}