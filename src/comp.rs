pub struct Compress{
    pub data: Vec<Vec<String>>,
}



impl Compress{

    pub fn split_and_comp(&mut self){

        for n in 0..=self.data[0].len(){

            let mut column = Vec::new();

            for val in self.data.iter(){
                if let Some(v) = val.iter().nth(n){
                    column.push(v.clone());
                }
            }
        

            let modified_col = self.remove_duplicates(column);


            for (index, val) in self.data.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = modified_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
        }
    }

    fn remove_duplicates(&self, mut col: Vec<String>) -> Vec<String>{

        let mut change_val = true;
        let mut prev_val = "";

        for v in &mut col.iter_mut(){

            if change_val{
                prev_val = v;
                change_val=false;
                continue;
            }

            if v == prev_val{
                *v = "-".to_string();
            }
            else{
                change_val=true;
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