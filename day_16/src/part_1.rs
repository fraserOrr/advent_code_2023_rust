use std::fs::File;
use std::usize;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use std::env;



fn main() -> Result<(),Box<dyn std::error::Error>>{
    //setup
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut file_path: String = "./src/".to_string();
    file_path.push_str(&args[1].to_string());
    let file = File::open(file_path)?;
    let  buf_reader = BufReader::new(file);

    //

    let mut char_map: Vec<Vec<char>> = vec![];
    let mut energised_char_map: Vec<Vec<char>> = vec![];
    for line in buf_reader.lines() {
        //println!("{:?}", line);
        char_map.push(line.as_ref().expect("nothing there").chars().collect());

        let empty_line: Vec<char> = vec!['.';line.as_ref().unwrap().len()-1];
        energised_char_map.push(empty_line)
    }



    char_map, energised_char_map = light_beam(0, 0, char_map, energised_char_map);


    for row in char_map.iter(){
        println!("{:?}",row);
    }
    for row in energised_char_map.iter(){
        println!("{:?}",row);
    }
    println!("time elapsed: {:?}", start.elapsed());
    Ok(())
    
}

fn light_beam(y_start: usize, x_start: usize, char_map: Vec<Vec<char>>,energised_char_map: Vec<Vec<char>>){


}
