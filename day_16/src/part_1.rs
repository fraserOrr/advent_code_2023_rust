use std::fs::File;
use std::iter::Skip;
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

        let empty_line: Vec<char> = vec!['.';line.as_ref().unwrap().len()];
        energised_char_map.push(empty_line)
    }

    let char_map = char_map;

    for row in char_map.iter(){
        println!("{:?}",row);
    }

    let mut starting_direction: char = 'R';
    let mut total_values: Vec<i32>= vec![];
    let clean_energy_map = energised_char_map.clone();
    for y in 0..char_map.len(){
        for x in 0..char_map[0].len(){
            if x == 0 || y == 0 || x == char_map[0].len()-1 || y == char_map.len()-1{
                if y == 0 {
                
                    starting_direction='D';
                }else if y == char_map.len()-1{
                    starting_direction='U';
                }else if x==0 {
                    starting_direction='R';
                }else if x==char_map[0].len()-1 {
                    starting_direction='L';
                }
                let mut beams: Vec<String> = vec![];
                energised_char_map=clean_energy_map.clone();
                println!("Beam starting from {},{} with dir {}",y,x,starting_direction);
                light_beam(y as isize, x as isize, starting_direction,&mut beams ,&char_map,&mut energised_char_map );

                let mut total = 0;
                //println!("---- energized map ----");
                for row in energised_char_map.iter(){
                    //println!("{:?}",row);
    
                    for tmp_char in row.iter(){
                        if tmp_char == &'#'{
                            total+=1;
                        }
                    }
                }
                println!("total is : {total}");
                total_values.push(total);
            }
            
        }
    }

    println!("executions done -- {}", {total_values.len()});
    let mut real_total=0;
    for total in total_values.iter(){
        if total > &real_total{
            real_total = total.clone();
        }
    }
    println!("highest total is {real_total}");
    println!("time elapsed: {:?}", start.elapsed());
    Ok(())
    
}



fn light_beam(mut y: isize,mut x: isize,mut direction: char, beams:&mut Vec<String> ,char_map: &Vec<Vec<char>>, energised_char_map:&mut Vec<Vec<char>>){
    let mut curr_beam = "Beam_".to_string();
    curr_beam.push_str(&y.to_string());
    curr_beam.push_str(&x.to_string());
    curr_beam.push_str(&direction.to_string());

    if beams.contains(&curr_beam){
        return;
    }else{
        beams.push(curr_beam);
    }
    //println!("New beam at {},{} going {}",y,x,direction);

    
    loop{
        if x < 0 || x >= char_map[0].len() as isize || y < 0 || y >= char_map.len() as isize{
            break;
            
        }
        
        
        let cur_char = char_map[y as usize][x as usize];

        
        energised_char_map[y as usize][x as usize] = '#';
    
    
        match cur_char {
            '.' => {
                match direction {
                    'R' => {
                        x +=1;
                        
                    },
                    'L' => {
                        x -=1;
                    },
                    'U' => {
                        y -=1;  
                    },
                    'D' => {
                        y+=1  
                    },
                    _ => {
                        panic!("failed to match direction")
                    }
                }
            },
            '/' =>{
                match direction {
                    'R' => {
                        y-=1;
                        direction='U';
                        
                    },
                    'L' => {
                        y+=1;
                        direction='D';
                    },
                    'U' => {
                        x+=1;
                        direction='R';
                    },
                    'D' => {
                        x-=1;
                        direction='L';
                    },
                    _ => {
                        panic!("failed to match direction")
                    }
                }
            },
            '\\' => {
                match direction {
                    'R' => {
                        y+=1;
                        direction='D';
                        
                    },
                    'L' => {
                        y-=1;
                        direction='U';
                    },
                    'U' => {
                        x-=1;
                        direction='L';
                    },
                    'D' => {
                        x+=1;
                        direction='R';
                    },
                    _ => {
                        panic!("failed to match direction")
                    }
                }
            },
            '|' => {
                match direction {
                    'R' => {
                        
                        light_beam(y +1, x,'D', beams, char_map, energised_char_map);
                        light_beam(y -1, x,'U', beams, char_map, energised_char_map);
                        return;
                        
                    },
                    'L' => {
                        
                        light_beam(y +1, x,'D', beams, char_map, energised_char_map);
                        light_beam(y -1, x,'U', beams, char_map, energised_char_map);
                        return;
                    },
                    'U' => {
                        y-=1;
                        
                    },
                    'D' => {
                        y+=1;
                    },
                    _ => {
                        panic!("failed to match direction")
                    }
                }
            },
            '-' =>{
                match direction {
                    'R' => {
                        x+=1;
                        
                    },
                    'L' => {
                        x-=1;
                    },
                    'U' => {
                        
                        light_beam(y, x+1,'R', beams, char_map, energised_char_map);
                        light_beam(y, x-1,'L', beams, char_map, energised_char_map);
                        return;
                    },
                    'D' => {
                        
                        light_beam(y, x+1,'R', beams, char_map, energised_char_map);
                        light_beam(y, x-1,'L', beams, char_map, energised_char_map);
                        return;
                    },
                    _ => {
                        panic!("failed to match direction")
                    }
                }
            },
            _ =>{
                panic!("illegal direction")
            }
    
        }


    }
    
    
    return;
}
