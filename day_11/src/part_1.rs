use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;


struct Galaxy{
    x: usize,
    y: usize,

}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let m_x = 140;
    let m_y = 140;

    let mut char_map: Vec<Vec<char>> = vec![];
    let mut galaxy_holder: Vec<Galaxy>= vec![];

    for (y,line) in buf_reader.lines().enumerate() {

        
    	char_map.push(line.expect("something there").chars().collect());
      
        //println!();
    }
    

    for row in char_map.iter(){
        if row.iter().find(|x| x==&&'.').unwrap().is_alphanumeric() == true{
            //ad a row of empty dots
            
        }
    }

    //how to find emtpy coloumns, check each vector at couloumn row, 

    for mut i in 0..char_map.len()-1{
        let mut empty = false;
        for row in char_map.iter(){
            if row.get(i).unwrap()==&'.'{
                empty=true;
            }
        }

        if empty == true{
            for row in char_map.iter(){
                char_map.get(j).unwrap().get(i).unwrap()
            }    
        }
    }
    for vec in char_map.iter(){
        for element in vec{
            print!("{:?}",element);
        }
        println!();
    }

   

    
    Ok(())

}
