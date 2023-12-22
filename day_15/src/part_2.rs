use std::fs::File;
use std::usize;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use std::env;



fn main() -> Result<(),Box<dyn std::error::Error>>{
    
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut string_vec: Vec<String> = vec![];

    for line in buf_reader.lines() {
       string_vec = line.expect("something there").split(",").map(|x| x.to_string()).collect();
    }
    
    

    let mut lensboxes: Vec<Vec<String>> = vec![];

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
            
            //println!("label: {} value: {} box_num {}",&label,&value,box_number);
            if lensboxes.get(box_number as usize).is_none(){
                let mut tmp_vec: Vec<String> = vec![];
                tmp_vec.push(box_entry);
                
                let tmp_box = tmp_vec;
                if box_number as usize >= lensboxes.len(){
                    while lensboxes.len()<=box_number as usize{
                        let tmp_box_2 = vec![];
                        lensboxes.push(tmp_box_2.clone())
                    }
                    lensboxes[box_number as usize] = tmp_box.clone();
                }else{
                    

                    lensboxes[box_number as usize] = tmp_box.clone();
                }
               
            }else{
                let mut box_contents = lensboxes.get_mut(box_number as usize).expect("box").clone();
                let mut contains = false;
                for cur_string in box_contents.clone().iter(){
                    let parts: Vec<String> = cur_string.clone().split(" ").map(|x| x.to_string()).collect();
                    if parts[0] == label{
                        let index = box_contents.iter().position(|r| r == cur_string).unwrap();
                        if args[1] == "--debug"{
                            println!("swapping {} from box {} : {:?}", &box_entry,&box_number,box_contents);
                        }
                        //
                        box_contents[index] = box_entry.clone();
                        lensboxes[box_number as usize] = box_contents.clone();
                        contains = true;
                    }
                }

                if contains == false{
                    let mut tmp_box = vec![box_entry.clone()];
                    lensboxes[box_number as usize].append(&mut tmp_box);
                }
            }

        }else if string.contains('-') == true {
            
            let value = parts[1].clone();
            let mut box_entry = label.clone();
            box_entry.push_str(" ");
            box_entry.push_str(&value);


            //println!("label: {} box_num: {}", box_entry, box_number);
            if lensboxes.get(box_number as usize).is_none(){
                //println!("nothing to do")
            }else{
                let mut box_contents = lensboxes.get_mut(box_number as usize).expect("box").clone();
                let mut removed = false;
                for cur_string in box_contents.clone().iter(){
                    let parts: Vec<String> = cur_string.clone().split([' ']).map(|x| x.to_string()).collect();
                    if parts[0] == label{
                        let index = box_contents.iter().position(|r| r.starts_with(&label)).unwrap();
                        if args[1] == "--debug"{
                            println!("removing {:?} with index of {} from box {}", &box_contents,&index,&box_number);
                        }
                        
                        box_contents.remove(index);
                        lensboxes[box_number as usize] = box_contents.clone();
                        removed = true;
                    }
                }
                if removed == false && args[1] == "--debug"{
                    println!("couldnt find {} in {:?}", box_entry,box_contents);
                }

               
            }

        }else{
            println!("opertaiton not found")
        }
        /* 
        for (y,tmp_box )in lensboxes.iter().enumerate(){
            println!("box: {} - {:?}",y, tmp_box);
        }
            */
    }
    for (y,tmp_box )in lensboxes.iter().enumerate(){
        println!("box: {} - {:?}",y, tmp_box);
    }
    let mut total: usize =0;
    for (y,tmp_box )in lensboxes.iter().enumerate(){
        for (x,lens) in tmp_box.iter().enumerate(){
            let value: Vec<String> = lens.clone().split(' ').map(|x| x.to_string()).collect();
            let value: String = value[1].clone();
            let value = value.parse::<usize>().expect("no number on split");
            let lens_total = (y+1) * (x+1) * value;            
            println!("box: {} slot: {} focal: {} == {}",(y+1),(x+1),value,lens_total);
            total += lens_total
        }
    }
    // 282274 too low = 286278
    println!("Total {total}");
    println!("time elapsed: {:?}", start.elapsed());
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