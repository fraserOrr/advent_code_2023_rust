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
        }else{
            total_perms += harder_permutation(compute_segments,computer_params)
        }
    }
   
    
    println!("Grand total: {}", total_perms);

   
    Ok(())

}
fn harder_permutation(compute_segments: Vec<String>,computer_params: Vec<i32> ) -> i32{
    //we need to break the big string dwon into possible blocks then user the other permutation functions
    let mut total_block_size: i32 = 0;
    for param in computer_params.iter(){
        total_block_size += param 
    }
    total_block_size += computer_params.len().clone() as i32 - 1;


    0
}
fn computer_permutation(compute_segments: Vec<String>,computer_params: Vec<i32> ) -> i32 {
   let mut total: i32 = 1;

   for i in 0..(computer_params.len()){
       let param = computer_params.get(i).unwrap();
       let segment = compute_segments.get(i).unwrap();
       
       let replaceables: Vec<&str> = segment.matches("?").collect();
       //println!("param {} and segment {}", param, segment);
       if replaceables.len() == segment.len(){
          
           total=total * ((segment.len() as i32 + 1)-param);
       }else{
            let returner = singler_permutation(segment.clone(), param.clone());

            total = total * returner
            
                  

       }
   }

   println!("total for this type is: {total}");
   return total
}

fn singler_permutation(segment: String, param: i32) -> i32 {
    let tmp: Vec<_> = segment. match_indices("#").collect();
    let f = tmp.first().unwrap().0 as i32;
    let l = tmp.last().unwrap().0 as i32;
    let span = 1 + (l-f);
        //println!("p: {}, f: {}, l: {}, span: {}, segment: {}", param,f,l,span,segment.len());
    if span < segment.len() as i32{
        let wiggle_room = param - (1 + (l-f));
         //  println!("wiggle room {}", wiggle_room);
        if wiggle_room > 0{
            let mut multi: i32 = 0;
            if f > 0{
                if wiggle_room < f +1 {
                        multi += wiggle_room;
                }else{
                        multi+= f + 1;
                }
            }
            if l < segment.len() as i32 -1 {
                if wiggle_room < (segment.len() as i32 - (l+1)){
                    multi+= wiggle_room
                }else{
                        multi+= segment.len() as i32 - (l+1);
                }
            }
            return multi
        }else{
            1
        }
    }else{
        1
    }
        

    
}