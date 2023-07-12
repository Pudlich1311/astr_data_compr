use csv::ReaderBuilder;
pub struct Csv{
    data: Vec<Vec<String>>
}

impl Default for Csv {
    fn default () -> Csv {
        Csv { data: Vec::new() }
    }
}

impl Csv{

    pub fn read_from_file(&mut self, path: String){

        self.data.clear();
        let mut rdr = csv::Reader::from_path(path).unwrap();
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


    pub fn read_from_string(&mut self, csv_string: String) -> std::io::Result<()>{

        self.data.clear();
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(csv_string.as_bytes());
        let mut records: Vec<Vec<String>> = Vec::new();
    
        for line in csv_string.split('\n') {
            let mut row = Vec::new();
            let mut record = csv::ByteRecord::new();
            if reader.read_byte_record(&mut record)? != false {
                for field in record.iter() {
                    let field_string = String::from_utf8_lossy(field).to_string();
                    row.push(field_string);
                }
                records.push(row);
            }
        }
        
        self.data = records;

        Ok(())
    }

    pub fn ret_data(&self) -> &Vec<Vec<String>>{
        return &self.data;
    }


}