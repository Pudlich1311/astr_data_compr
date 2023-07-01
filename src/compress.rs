use std::collections::HashSet;

pub struct Compress{
    pub data: Vec<Vec<String>>,
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
            let duplicates_col = self.remove_duplicates(round_col);


            for (index, val) in self.data.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = duplicates_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
        }
    }


    fn round_values(&self, mut col: Vec<String>) -> Vec<String>{

        for num in &mut col.iter_mut() {

            if num == ""{
                continue;
            }

            let number: Result<f64, _> = num.parse();

            match number {

                Ok(value) => {
                    if  value.fract() != 0.0 {
                        //Find best fitting round value
                        let power_of_10 = 10_f64.powi(1);
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


    // pub fn print(&self){


    //     for val in self.data.iter(){
    //         for v in val.iter(){
    //             print!("'{}',",v );
    //         }
    //         println!("NEXT");
    //     }

    // }
    

}