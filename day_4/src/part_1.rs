use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut total_p = 0;
    let mut round_p=0;

    for line in buf_reader.lines() {
        round_p=0;
    	let content = line.expect("oops file error");
    	//println!("{}",content);

        let parts_of_content: Vec<&str> = content.split("|").collect();

        let tmp: Vec<&str> = parts_of_content[0].split(":").collect();
        
        let lotto_nmb_list: Vec<&str> = tmp[1].split(" ").collect();

        let match_nmb_list: Vec<&str> = parts_of_content[1].split(" ").collect();

    	
        for match_nmb in match_nmb_list.iter(){

            for lotto_nmb in lotto_nmb_list.iter(){
                match lotto_nmb.is_empty() {
                    true => {

                    }
                    false => {
                       match lotto_nmb == match_nmb {
                            true => {
                                match round_p == 0{
                                    true => {
                                        round_p = 1;
                                    }
                                    false => {
                                        round_p = round_p * 2;
                                    }
                                }
                            }
                            false => {

                            }
                        }
                    }
                }
 
            }
        }

        println!("this round won: {}", &round_p);
        total_p = total_p + round_p;
    }

    

    println!("Total p from cards are: {}", total_p);
    
    Ok(())

}
