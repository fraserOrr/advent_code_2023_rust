use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut string_vec: Vec<String> = vec![];

    for line in buf_reader.lines() {
       string_vec = line.expect("something there").split(",").map(|x| x.to_string()).collect();
    }
    
    let mut total: i64 =0;
    for string in string_vec.iter(){
        println!("{:?}",string);
        total += hashing_function_p1(string);
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