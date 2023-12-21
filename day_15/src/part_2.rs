use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;




fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut string_vec: Vec<String> = vec![];

    for line in buf_reader.lines() {
       string_vec = line.expect("something there").split(",").map(|x| x.to_string()).collect();
    }
    
    let mut total: i64 =0;

    let mut lensboxes: Vec<Vec<String>> = Vec::with_capacity(255);

    for string in string_vec.iter(){
        //println!("{:?}",string);
        //total += hashing_function_p1(string);
        let parts: Vec<String> = string.clone().split(['-','=']).map(|x| x.to_string()).collect();
        let label = parts[0].clone();
        let box_number = hashing_function_p1(&label);
        
        if string.contains('=') == true{
            let value = parts[1].clone();
            let mut box_entry = label.clone();
            box_entry.push_str(" ");
            box_entry.push_str(&value);
            
            println!("label: {} value: {} box_num {}",&label,&value,box_number);
            if lensboxes.get(box_number as usize).is_none(){
                let mut tmp_vec: Vec<String> = vec![];
                tmp_vec.push(box_entry);
                
                let tmp_box = tmp_vec;
                if box_number as usize >= lensboxes.len(){
                    lensboxes.push(tmp_box);
                }else{
                    lensboxes.insert(box_number as usize, tmp_box);
                }
               
            }

        }else if string.contains('-') == true {
            println!("label: {} box_num: {}",label, box_number);
            

        }else{
            println!("opertaiton not found")
        }

    }
     
    for tmp_box in lensboxes.iter(){
        println!("{:?}", tmp_box);
    }
    println!("Total {total}");
    Ok(())

}

fn hashing_function_p1(input: &String) -> i64{
    let mut curr_value: i64 = 0;
    for curr_char in input.chars().into_iter(){
        curr_value += curr_char as i64;
        curr_value *= 17;
        curr_value = curr_value % 256;

    }



    return  curr_value;
}