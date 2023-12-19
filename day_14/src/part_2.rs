use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/*
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}*/

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut char_map: Vec<Vec<char>> = vec![];
    let mut value_mapping: HashMap<Vec<Vec<char>>,i64> = HashMap::new();

    for line in buf_reader.lines() {
       char_map.push(line.expect("something there").chars().collect());
    }

    for (x,row) in char_map.iter().enumerate(){
        println!("row: {} : {:?}",x,row)
    }
    let mut found_loop = false;
    let mut hash_pointer = 0;
    let mut pos = 0; 
    for loop_count in 0..1000000000{
        if found_loop == false{
            char_map = one_loop(char_map);
            let mut total: i64 = 0;
            for (x ,row) in char_map.iter().enumerate(){
                let x: i64 = char_map.len() as i64 - x as i64;
 
                total += x * row.iter().filter(|&cha| *cha == 'O').count() as i64

            }
            //calcl hash


            if value_mapping.contains_key(&char_map) == false{
                value_mapping.insert(char_map.clone(),total);
            }else{
                let vec: Vec<Vec<Vec<char>>> = value_mapping.clone().into_keys().collect();
                for (count, key ) in vec.iter().enumerate(){
                    if key == &char_map{
                        pos = count;
                        break;
                    }

                }
                println!("Hash found, we have hit a loop at {} with pos: {}", loop_count, pos);
                found_loop=true;
                hash_pointer= pos;
            }


        }else{
            hash_pointer +=1;
            if hash_pointer >= value_mapping.len(){
                hash_pointer = pos
            }
        }
        
        if loop_count % 100000000 == 0{
            println!("{}",loop_count / 10000000 )
        }

        if loop_count < 10{
            println!("---------{loop_count}-----------");
            for row in char_map.iter(){
                
                println!("{:?}",row);
            }
        }
    }
    

    print!("Hash pointer landed on: {hash_pointer}");
    let vec: Vec<i64> = value_mapping.into_values().collect();
    
    for (i,value) in vec.iter().enumerate(){
        println!("{i} {value}");
        if i == hash_pointer{
            println!("End value is {}", value);
            break;
        }
    }

     
    Ok(())

}

fn one_loop(mut char_map: Vec<Vec<char>>) ->  Vec<Vec<char>>{
//move everything up
    for y in 0..char_map.len(){
        if y > 0{
            for x in 0..char_map.get(y).unwrap().len(){
                if char_map[y][x]  == 'O'{
                    for i in (0..y).rev(){
                        if char_map[i][x] == '.'{
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
   for y in 0..char_map.len(){
        for x in 1..char_map.get(y).unwrap().len(){
            if char_map[y][x] == 'O'{
                for i in (0..x).rev(){
                    if char_map[y][i] == '.'{
                        char_map[y].swap(i,i+1);
                    }else{
                        break;
                    }
                }
            }
        }
       
    }
    for y in (0..char_map.len()).rev(){
        for x in 0..char_map.get(y).unwrap().len(){
            if char_map[y][x] == 'O'{
                for i in y..char_map.len()-1{
                    if char_map[i+1][x] == '.'{
                            char_map[i+1][x] = 'O';
                            char_map[i][x]= '.';
                    }else{
                        break;
                    }
                }
            }
        }
        
    }

    for y in 0..char_map.len(){
        for x in (0..char_map.get(y).unwrap().len()).rev(){
            if char_map[y][x] == 'O'{
                for i in x..char_map.get(y).unwrap().len()-1{
                    if char_map[y][i+1] == '.'{
                        char_map[y].swap(i,i+1);
                    }else{
                        break;
                    }
                }
            }
        }

    }/*
    println!("---------RIGHT-----------");
    */
    return char_map
}