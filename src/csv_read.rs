
pub struct CsvRead{
    headers: Vec<String>,
    values: Vec<Vec<String>>
}

impl Default for CsvRead {
    fn default () -> CsvRead {
        CsvRead { headers: Vec::new(), values: Vec::new() }
    }
}

impl CsvRead{
    

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

    pub fn print(&self){

        for header in &self.headers{
            print!("{} ",header);
        }

        for val in self.values.iter(){
            for v in val.iter(){
                print!("{} ",v );
            }
            break;
        }

    }

    pub fn ret_values(&self) -> &Vec<Vec<String>>{
        return &self.values;
    }

    pub fn ret_head_size(&self) -> usize{
        return self.headers.len()
    }
}