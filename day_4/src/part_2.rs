use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    
    let mut round_p:usize=0;
    let mut game_score: Vec<usize> = Vec::new();

    for (j,line) in buf_reader.lines().enumerate() {
        round_p=0;
        match game_score.get(j) == None {
            true => game_score.push(1),
            false => (),
        }
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
                                
                                round_p += 1;
                             
                            }
                            false => {

                            }
                        }
                    }
                }
 
            }
        }
        
        println!("round {} won: {} scratchcards * {}",&j, &round_p,game_score.get(j).expect("there was an error getting wins for round"));
        round_p = round_p ;
        for x in j+1..j+1 + round_p{
            match game_score.get(x) == None {
                true => game_score.push(1),
                false => (),
            }
            game_score[x] = game_score[x] + (1* game_score.get(j).expect("there was an error getting wins for round")); 
            println!("added {} to card {}, you know have {}",game_score.get(j).expect("there was an error getting wins for round"), x,game_score.get(x).expect("there was an error getting wins for round") );
        }
        
    }
    let mut total_p:usize = 0;
    for (x,item) in game_score.iter().enumerate(){
        println!("for card {} you had {}",x+1,item);
        total_p += item;
    }

    println!("Total p from cards are: {}", total_p);
    
    Ok(())

}
