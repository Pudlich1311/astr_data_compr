use std::collections::HashSet;
use rust_decimal::prelude::*;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use zfp_sys::*;


pub struct Compress{
    pub data: Vec<Vec<String>>,
    pub compressed_data: Vec<Vec<String>>,
    pub round: bool
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

            if(self.round){
                column = self.round_values(column);
            }


            let delta_col = self.delta_encode(&column);
            let duplicates_col = self.remove_duplicates(delta_col);
        
            for (index, val) in self.data.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = duplicates_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
            self.compressed_data=self.data.clone();
        }
    }

    


    fn float_encode(&self, col: Vec<String>) -> Vec<String>{

        let mut encoded_values = self.delta_encode(&col);


        return encoded_values;

    }



    fn delta_encode(&self, data: &Vec<String>) -> Vec<String>{

        let mut encoded_data = Vec::with_capacity(data.len());
        let mut prev_value = Decimal::new(0, 2);
    
        for value in data {

            if Decimal::from_str(&value).is_err(){
                encoded_data.push(value.to_string());
                continue;
            }
            let temp = Decimal::from_str(&value).unwrap();
            let diff = temp - prev_value;

            encoded_data.push(diff.to_string());
            prev_value = temp;
        }
    
        encoded_data
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

        let mut prev_val = String::new();
        let mut first_iter = true;

        for num in &mut col.iter_mut() {
    
            if num == ""{
                continue; 
            }
    
            if first_iter{
                first_iter=false;
                prev_val=num.to_string();
                continue;
            }

            if num == &prev_val{
                *num="-".to_string();
            }
            else{
                prev_val=num.to_string();
            }

        }

        return col;
    }

}