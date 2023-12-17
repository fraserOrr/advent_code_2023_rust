use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;


struct Galaxy{
    x: i64,
    y: i64,

}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
     
    let mut char_map: Vec<Vec<char>> = vec![];
    let mut galaxy_holder: Vec<Galaxy>= vec![];
    let mut massive_row: Vec<usize>= vec![];
    let mut massive_coloumn: Vec<usize>= vec![];
    for line in buf_reader.lines() {

        
    	char_map.push(line.expect("something there").chars().collect());
      
        //println!();
    }
    
    
    for (x ,row) in char_map.iter().enumerate(){
        if row.iter().find(|x| x==&&'#').is_none() == true{
            //ad a row of empty dots
            println!("empty row at {}",x.clone());
            
            massive_row.push(x);
            
            //check if this fucks up passing through it
        }
    }

    


    //how to find emtpy coloumns, check each vector at couloumn row, 
    let mut i = 0;
    while i < (char_map.len()){
        let mut empty = true;
        for row in char_map.iter(){
            if row.get(i).unwrap()==&'#'{
                empty=false;
            }
        }

        if empty == true{
            println!("empty coloumn at {}",i.clone());
            massive_coloumn.push(i.clone());    
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
    /**/
    for vec in char_map.iter(){
        println!("{:?}",vec)
    }
    for (c,node) in galaxy_holder.iter().enumerate(){
        println!("Node {} at {} : {}",c,node.y,node.x);
    }
      println!("massive row: {:?}",massive_row);
          println!("massive col: {:?}",massive_coloumn);

          //*/
    let mut counter = 0;
    let mut total = 0;
    
    //let expanison = 2 ;
    let expanison = 1000000;
    while counter < galaxy_holder.len() {

        for x in counter+1..(galaxy_holder.len()){
          
            let mut multi = 0;
            for tmp in massive_row.iter(){
                let big_row = tmp.clone() as i64;

                if ( galaxy_holder.get(counter).unwrap().y < big_row && big_row < galaxy_holder.get(x).unwrap().y)  {
                    multi += expanison -1
                }

            }

            for tmp in massive_coloumn.iter(){
                let big_col = tmp.clone() as i64;

                if (galaxy_holder.get(counter).unwrap().x < big_col && big_col < galaxy_holder.get(x).unwrap().x)  {
                    multi += expanison -1
                }else if galaxy_holder.get(x).unwrap().x < big_col && big_col < galaxy_holder.get(counter).unwrap().x{
                    multi += expanison -1
                }

            }
            let x_gap:i64 = ((galaxy_holder.get(counter).unwrap().x) - (galaxy_holder.get(x).unwrap().x )).abs() ;
            let  y_gap:i64 = ((galaxy_holder.get(counter).unwrap().y) - (galaxy_holder.get(x).unwrap().y )).abs() ;
            let gap =x_gap + y_gap + multi; 
            //println!("gap between {} and {} is {} ",counter,x,gap);
            total += gap;
        }

        counter+=1;
    }

    //515598573144 is too high   267405573144 is to low
    //504715068438
    println!("Pair total: {}", total);
    
    Ok(())

}
