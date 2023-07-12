use std::collections::HashSet;
use rust_decimal::prelude::*;
use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;


pub struct Decompress{
    pub data: Vec<Vec<String>>,
}

impl Decompress{

    pub fn decompress(&mut self, input: Vec<u8>)-> String{

        let mut decoder = ZlibDecoder::new(&input[..]);
        let mut decompressed_data = String::new();
        decoder.read_to_string(&mut decompressed_data);

        decompressed_data
    }

    pub fn decode(&mut self){

        for n in 0..=self.data[0].len(){

            let mut column = Vec::new();

            for val in self.data.iter(){
                if let Some(v) = val.iter().nth(n){
                    column.push(v.clone());
                }
            }

            let duplicates_col = self.duplicates(column);
            let delta_col = self.delta_decode(duplicates_col);

            

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

    fn delta_decode(&self, mut col: Vec<String>) -> Vec<String>{

        let mut first_iter = true;
        let mut prev_value= Decimal::new(0, 2);
        let mut temp= Decimal::new(0, 2);
        for num in &mut col.iter_mut() {

            if num == ""{
                continue;
            }

            let can_parse=Decimal::from_str(num).is_ok();

            if can_parse{


                temp = Decimal::from_str(num).unwrap();

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
    


}