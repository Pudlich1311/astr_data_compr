pub struct Compress{
    pub values: Vec<Vec<String>>,
}



impl Compress{

    pub fn split(&mut self){

        for n in 0..=self.values[0].len(){

            let mut column = Vec::new();

            for val in self.values.iter(){
                if let Some(v) = val.iter().nth(n){
                    column.push(v.clone());
                }
            }
        

            let modified_col = self.duplicates(column);
            //here do some stuff on columns


            for (index, val) in self.values.iter_mut().enumerate(){
                if let Some(v) = val.iter_mut().nth(n){
                    let value = modified_col.get(index).to_owned();
                    *v = value.unwrap().to_owned();
                }
            }
            
        }
    }

    fn duplicates(&self, mut col: Vec<String>) -> Vec<String>{

        let mut change_val = true;
        let mut prev_val = "";

        for v in &mut col.iter_mut(){

            if change_val{
                prev_val = v;
                change_val=false;
                continue;
            }

            if v == prev_val{
                *v = "".to_string();
            }
            else{
                change_val=true;
            }
        }
        return col;
    }


    pub fn print(&self){


        for val in self.values.iter(){
            for v in val.iter(){
                print!("'{}',",v );
            }
            println!("NEXT");
        }

    }
    

}