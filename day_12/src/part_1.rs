use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut total_perms = 0;



    for (y,line) in buf_reader.lines().enumerate() {
        let re = Regex::new(r"([?#]+)").unwrap();
        let re_num = Regex::new(r"([0-9]+)").unwrap();
        let line_segments: Vec<String>= line.expect("bad file line").split(" ").map(|x| x.to_string()).collect();
        
         //println!("{:?}", line_segments);
        

       
        let compute_segments: Vec<String> = re.find_iter(line_segments.get(0).expect("String")).map(|x|  x.as_str().to_string()).collect();
        let computer_params: Vec<i32> = re_num.find_iter(line_segments.get(1).expect("String")).map(|x|  x.as_str().to_string().parse::<i32>().expect("not num")).collect();
        println!("{:?}", computer_params);
        println!("{:?}",compute_segments);
            
        //work out combinations per compute segement 

        if compute_segments.len() == computer_params.len() {
            total_perms += computer_permutation(compute_segments,computer_params)
        }
    }
   
    
    

   
    Ok(())

}

fn computer_permutation(compute_segments: Vec<String>,computer_params: Vec<i32> ) -> i32 {
   let mut total: i32 = 1;

   for i in 0..(computer_params.len()-1){
       let param = computer_params.get(i).unwrap();
       let segment = compute_segments.get(i).unwrap();
       
       let replaceables: Vec<&str> = segment.matches("?").collect();
       
       if replaceables.len() == segment.len(){
          
           total=total * ((segment.len() as i32 + 1)-param);
       }else{
         let tmp: Vec<_> = segment. match_indices("#").collect();
         let f = tmp.first().unwrap().0;
         let l = tmp.last().unwrap().0;
         let span = 1 + (l-f);
         if span < segment.len(){
             total=total * (f);
         }

       }
   }

   println!("{total}");
   return total
}