use std::fs::File;
use  std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use array2d::{Array2D, Error};

struct Number {
    x: usize,
    y: usize,
    length: usize,
    value: i32,
}

struct SpecChar{
    x: usize,
    y: usize,
    value: char,
}
fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    let m_x = 140;
    let m_y = 140;

    let mut char_map = Array2D::filled_with('.',m_x,m_y);

    for (y,line) in buf_reader.lines().enumerate() {

        
    	let  line_content = line.expect("oops file error");
        for (x , tmp_char) in line_content.chars().enumerate() {
            if tmp_char != '.' {
                //println!("x:{},y{}",&x,&y);
                let result = char_map.set(y,x,tmp_char);
                assert_eq!(result, Ok(()));

            };
        };

        
    }

    //print map
        //print map
    for y in 0..m_x{
        for x in 0..m_y{
            let cur_char = char_map.get(y,x).expect("out of range");
            print!("{:?}",cur_char);
            
        };
        println!();
    };
    //vector of numbers

    let mut number_holder: Vec<Number> = Vec::new();

    //vector of special locations
    let mut spec_char_holder: Vec<SpecChar> = Vec::new();


    //get things
    let mut y = 0;
    let mut x = 0;

    while y < m_y{
        while x < m_x{

            let cur_char = char_map.get(y,x).expect("out of range");
            //print!("{:?}",cur_char);
            if cur_char != &'.' {
                //we have a character we want to mark
                match cur_char.is_numeric() {
                    true => {
                        //check if length of number
                        let mut current_number = Vec::new();
                        //println!("found number: {}",&cur_char);
                        current_number.push(cur_char.to_string().parse::<i32>().unwrap());
                        let mut tmp_x = x+1;
                        while tmp_x < m_x {
                            let next_char = char_map.get(y,tmp_x).expect("out of range");
                            //println!("next to check is: {}", &next_char);
                            if  next_char.is_numeric() == false{
                                //println!("EON");
                                break;
                            } else {

                                let next_num = char_map.get(y,tmp_x).expect("out of range").to_string().parse::<i32>().unwrap() ;
                               //println!("found connecting num: {}", &next_num);
                               current_number.push(next_num);
                               tmp_x += 1;
                            }



                        };

                        let mut multiplyer:i32 = 1;
                        let mut tmp = 0;
                        let buffer_size = current_number.len();
                        //println!("buffer size: {}",buffer_size );
                        for j in 0..buffer_size {
                            let tmp_convert = current_number.pop().unwrap();

                            tmp = tmp + (tmp_convert * multiplyer);
                            //println!(" {} x {}", &tmp, &multiplyer);
                            multiplyer = multiplyer * 10;
                            
                        };

                        //println!("made num {}", tmp);

                        let current_item = Number{
                                x: x,
                                y: y,
                                length: buffer_size,
                                value: tmp,
                        };
                        number_holder.push(current_item);

                        x = x + buffer_size -1;
                         //println!("x skipped by: {}", buffer_size);
                    }
                    false => {
                        //println!("spec char: {}",&cur_char);
                        if cur_char == &'*'{
                            let tmp_spec = SpecChar{
                                    x: x,
                                    y: y,
                                    value: cur_char.clone(),

                            };
                            spec_char_holder.push(tmp_spec)
                        }
                        
                    }

                }
            }
            x+=1;
            //println!("x ++");
        };

        y = y+1;
        x = 0;
       //println!("y ++");
    };
    //
    // we should have a list op important things

    for x in number_holder.iter(){
        println!("number {}, starting at {},{}",x.value,x.y,x.x);
    }
    for x in spec_char_holder.iter(){
        println!("spec char {}, starting at {},{}",x.value,x.y,x.x);
    }


    //work out if something is connected 

    let mut total=0;
    
    for curr_spec in spec_char_holder.iter() {
        let mut near_numbers=0;
        let mut gear_total=0;
        for number in number_holder.iter(){
            if number.x == 0{
                if (number.x) <= curr_spec.x && curr_spec.x <= (number.x + number.length   ) {
                  
                    if number.y == 0{
                        if number.y  <= curr_spec.y && curr_spec.y  <= number.y + 1 {
                            println!("1 number: {} is valid from char {}",number.value,curr_spec.value);
                            if near_numbers == 0{
                                near_numbers = near_numbers +1;
                                gear_total = number.value;
                            }else{
                                near_numbers = near_numbers +1;
                                gear_total = gear_total * number.value;
                            }
                            
                           
                        };
                    }else{
                        if number.y -1 <= curr_spec.y && curr_spec.y  <= number.y + 1 {
                            println!("2 number: {} is valid from char {}",number.value,curr_spec.value);
                            if near_numbers == 0{
                                near_numbers = near_numbers +1;
                                gear_total = number.value;
                            }else{
                                near_numbers = near_numbers +1;
                                gear_total = gear_total * number.value;
                            }
                            
                        };
                    }
                    
                };
            }else{
                if (number.x -1) <= curr_spec.x && curr_spec.x <= (number.x + number.length   ) {
                    //println!("within x range");
                  if number.y == 0{
                        if number.y  <= curr_spec.y && curr_spec.y  <= number.y + 1 {
                            println!("3 number: {} is valid from char {}",number.value,curr_spec.value);
                            if near_numbers == 0{
                                near_numbers = near_numbers +1;
                                gear_total = number.value;
                            }else{
                                near_numbers = near_numbers +1;
                                gear_total = gear_total * number.value;
                            }
                            
                        };
                    }else{
                        if number.y -1 <= curr_spec.y && curr_spec.y  <= number.y + 1 {
                            println!("4 number: {} is valid from char {}",number.value,curr_spec.value);
                            if near_numbers == 0{
                                near_numbers = near_numbers +1;
                                gear_total = number.value;
                            }else{
                                near_numbers = near_numbers +1;
                                gear_total = gear_total * number.value;
                            }
                            
                        };
                    }
                };
            }
            


        
        };


        //
        if near_numbers >=2 {
            println!("how many matched {near_numbers}, with value: {gear_total}" );
            total= total + gear_total;
        }
        
    };
    println!("total: {total}");
    Ok(())

}
