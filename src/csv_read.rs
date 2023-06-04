use std::{fs::File, fmt::write, io::Write};


pub struct Csv{
    headers: Vec<String>,
    values: Vec<Vec<String>>
}

impl Default for Csv {
    fn default () -> Csv {
        Csv { headers: Vec::new(), values: Vec::new() }
    }
}

impl Csv{
    

    pub fn read(&mut self){
        let mut rdr = csv::Reader::from_path("/home/pudlich/Documents/projekty/ntwi/astr_data_compr/test.csv").unwrap();
        
        for header in rdr.headers().unwrap().iter(){
            self.headers.push(header.to_string());
        }

        for row in rdr.records() {
            let mut temp = Vec::new();
            for r in row.unwrap().iter() {
                temp.push(r.to_string());
            }
            self.values.push(temp);
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

        let data_str = self.headers.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(",");
        write!(file,"{}\n",data_str);

        for val in modified_data.iter(){
            let line = val.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(",");
            write!(file,"{}\n",line);
        }

    }

    pub fn ret_values(&self) -> &Vec<Vec<String>>{
        return &self.values;
    }


}