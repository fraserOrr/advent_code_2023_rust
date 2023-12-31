use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;


struct Galaxy{
    x: i64,
    y: i64,

}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
     
    let mut char_map: Vec<Vec<char>> = vec![];
    let mut galaxy_holder: Vec<Galaxy>= vec![];

    for line in buf_reader.lines() {

        
    	char_map.push(line.expect("something there").chars().collect());
      
        //println!();
    }
    
    let mut insert_map: Vec<usize> = vec![];
    for (x ,row) in char_map.iter().enumerate(){
        if row.iter().find(|x| x==&&'#').is_none() == true{
            //ad a row of empty dots
            println!("empty row at {}",x.clone());
            insert_map.push(x);
            
            //check if this fucks up passing through it
        }
    }

    if insert_map.is_empty() == false{
        for (y,x) in insert_map.iter().enumerate(){
            let empty_row: Vec<char> = vec!['.';char_map.first().unwrap().len()];
            char_map.insert(x.clone() + y,empty_row);
        }
    }


    //how to find emtpy coloumns, check each vector at couloumn row, 
    let mut i = 0;
    while i <= (char_map.len()-1){
        let mut empty = true;
        for row in char_map.iter(){
            if row.get(i).unwrap()==&'#'{
                empty=false;
            }
        }

        if empty == true{
            println!("empty coloumn at {}",i.clone());
            for row in char_map.iter_mut(){
                
                row.insert(i,'.');
                
                
            }   
            i+=1;
            
        }
        i+=1;
    }


    for (y,vec) in char_map.iter().enumerate(){
        for (x,element) in vec.iter().enumerate(){
            if element == &'#'{
                let tmp_x = x as i64;
                let tmp_y = y as i64;
                galaxy_holder.push(Galaxy{x:tmp_x.clone(),y:tmp_y.clone()})
                
            }
        }
        
    }

    for vec in char_map.iter(){
        println!("{:?}",vec)
    }
    for (c,node) in galaxy_holder.iter().enumerate(){
        println!("Node {} at {} : {}",c,node.y,node.x);
    }
   
    let mut counter = 0;
    let mut total = 0;
    while counter < galaxy_holder.len() {
      //println!();
      //println!(" {} ",counter);
      for x in counter..(galaxy_holder.len()){
          //print!(" {} ", x);
          let x_gap:i64 = ((galaxy_holder.get(counter).unwrap().x) - (galaxy_holder.get(x).unwrap().x )).abs() ;
          let  y_gap:i64 = ((galaxy_holder.get(counter).unwrap().y) - (galaxy_holder.get(x).unwrap().y )).abs() ;
          let gap =x_gap + y_gap; 
          println!("gap between {} and {} is {} ",counter,x,gap);
          total += gap;
      }

      counter+=1;
    }


    println!("Pair total: {}", total);
    
    Ok(())

}
