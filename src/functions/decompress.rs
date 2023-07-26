use std::collections::HashSet;
use rust_decimal::prelude::*;
use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;


pub struct Decompress{
    pub data: Vec<Vec<String>>,
}

impl Decompress{


    pub fn decode(&mut self){

        for n in 0..=self.data[0].len(){
            
            let mut column = Vec::new();
            
            for val in self.data.iter(){
                if let Some(v) = val.iter().nth(n){
                    column.push(v.clone());
    }
            }
    
            let duplicates_col = self.duplicates(column);
            let delta_col = self.delta_decode(&duplicates_col);


        
            for (index, val) in self.data.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = delta_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
        }
    }

    fn duplicates(&self, mut col: Vec<String>) -> Vec<String>{

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
    
            if num =="-"{
                *num=prev_val.to_string();
    }
            else{
                prev_val=num.to_string();
            }
        }
    
        return col;
    }

    fn delta_decode(&self, encoded_data: &Vec<String>) -> Vec<String>{

        let mut decoded_data = Vec::with_capacity(encoded_data.len());
        let mut prev_value = Decimal::new(0, 2);
    
        for diff in encoded_data {

            if Decimal::from_str(&diff).is_err(){
                decoded_data.push(diff.to_string());
                continue;
            }
            let temp = Decimal::from_str(&diff).unwrap();
            let value = prev_value + temp;
            decoded_data.push(value.to_string());
            prev_value = value;
        }
    
        decoded_data
    }


    


}