use std::collections::HashSet;
use rust_decimal::prelude::*;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;


pub struct Compress{
    pub data: Vec<Vec<String>>,
    pub compressed_data: Vec<u8>
}

impl Compress{

    pub fn split_and_compress(&mut self){

        for n in 0..=self.data[0].len(){

            let mut column = Vec::new();

            for val in self.data.iter(){
                if let Some(v) = val.iter().nth(n){
                    column.push(v.clone());
                }
            }

            
            let round_col = self.round_values(column);
            let delta_col = self.delta_encode(round_col);
            let duplicates_col = self.remove_duplicates(delta_col);
            

            for (index, val) in self.data.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = duplicates_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
        }


        let flattened_data: String = self.data
        .iter()
        .map(|inner| inner.join(","))
        .collect::<Vec<String>>()
        .join("\n");


        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(flattened_data.as_bytes()).unwrap();
        self.compressed_data = encoder.finish().unwrap();

    }

    fn delta_encode(&self, mut col: Vec<String>) -> Vec<String>{

        let mut first_iter = true;
        let mut prev_value= Decimal::new(0, 2);
        let mut temp= Decimal::new(0, 2);
        for num in &mut col.iter_mut() {

            if num == ""{
                continue;
            }

            let can_parse=Decimal::from_str(num).is_ok();

            if can_parse{

                let scientific = Decimal::from_scientific(num).is_ok();
                if scientific{
                    temp =  Decimal::from_scientific(num).unwrap();
                }
                else{
                    temp = Decimal::from_str(num).unwrap();
                }

                if first_iter{
                    prev_value=temp;
                    first_iter=false;
                    continue;
                }

                let delta = prev_value - temp;
                *num=delta.to_string();
            }

        }

        
        return col;
    }

    fn round_values(&self, mut col: Vec<String>) -> Vec<String>{

        let mut power_of_10 = 10_f64.powi(6);

        for num in &mut col.iter_mut() {

            if num == ""{
                continue;
            }

            if num.to_lowercase().contains("error"){
                power_of_10 = 10_f64.powi(2);
            }
            let number: Result<f64, _> = num.parse();

            match number {

                Ok(value) => {
                    if  value.fract() != 0.0 {
                        let rounded = (value * power_of_10).round() / power_of_10;
                        *num=rounded.to_string();
                    }
                }
                Err(e) => {
                    continue;
                }
            }
        }

        return col;
    }

    fn remove_duplicates(&self, mut col: Vec<String>) -> Vec<String>{

        let mut encountered = HashSet::new();

        for num in &mut col.iter_mut() {

            if num == ""{
                continue;
            }

            if !encountered.insert(num.clone()) {
                *num = "-".to_string();
            }
        }

        return col;
    }

}