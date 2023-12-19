use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut char_map: Vec<Vec<char>> = vec![];


    for line in buf_reader.lines() {
       char_map.push(line.expect("something there").chars().collect());
    }

    for (x,row) in char_map.iter().enumerate(){
        println!("row: {} : {:?}",x,row)
    }
   
    
    //move everything up
    for y in 0..char_map.len(){
        if y > 0{
            for x in 0..char_map.get(y).unwrap().len(){
                if char_map.get(y).unwrap().get(x).unwrap().clone() == 'O'{
                    for i in (0..y).rev(){
                        if char_map.get(i).unwrap().get(x).unwrap() == &'.'{
                            //println!("swapping x: {}, y: {} with position x: {}, y: {}",x,i+1,x,i);
                            char_map[i][x] = 'O';
                            char_map[i+1][x]= '.';
                        }else{
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("--------------------");
    let mut total = 0;
    for (x,row) in char_map.iter().enumerate(){
        let x = char_map.len()-x;
        println!("row: {} : {:?}",x,row);
        total += x * row.iter().filter(|&cha| *cha == 'O').count()

    }
   
    println!("Toal: {total}");
    Ok(())

}