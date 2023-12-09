
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;


struct Hand{
    cards: Vec<char>,
    hand_rank: i64,
    bet: i64,
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/test.txt")?;
    let  buf_reader = BufReader::new(file);
    let card_strength: Vec<&str> = Vec::from(["A","K","Q","J","T","9","8","7","6","5","4","3","2"]);

    let mut hand_container: Vec<Hand> = vec![];
    for (_count,line) in buf_reader.lines().enumerate() {
       //println!("Hand: {}",count);
       let content = line.expect("oops file error");
       let tmp: Vec<&str>  = content.split(" ").collect();
      
       let tmp_hand = Hand{
           
           cards: tmp[0].parse::<String>().expect("bad card string").to_string().chars().collect(),
           hand_rank: calculate_hand(tmp[0].parse::<String>().expect("bad card string"),&card_strength),
           bet: tmp[1].parse::<i64>().expect("bad bet"),
           
       };
       hand_container.push(tmp_hand);
    }

    
    hand_container.sort_unstable_by_key(|hand| (hand.hand_rank));

    let swapped = false;
    loop{
        for i in 0..hand_container.len()-2{
            let card1 =  hand_container.get(i).unwrap();
            let card2 =  hand_container.get(i+1).unwrap().clone();
            if card1.hand_rank == card2.hand_rank{
                let sorted = false;
                while !sorted{
                    if card1.cards[0] > card2.cards[0]{
                        hand_container.swap(i,i+1);
                    }
                }
            }
        }

        if swapped ==true{
            break;
        }
    }

    let mut total = 0;
    for (x,hand) in hand_container.iter().enumerate().rev(){
        let i = x as i64;
        
        let rank: i64 = i +1 ;
        let score = hand.bet * rank;
        total= total + score;
        println!("in rank {:0>3} with hand: {:?}  --> {} they placed: {:0>4} and scored: {}",rank,hand.cards,hand.hand_rank,hand.bet,score);
    }
    
    //249661593
    println!("total: {}",total);
    Ok(())
}

fn calculate_hand(hand : String,card_order: &Vec<&str>) ->i64{
    let mut card_calc: HashMap<&str,i64> = HashMap::new();
    
    let hand_capt: Vec<&str>  = hand.split("").collect();
    let base: i64 = 10;
    let mut hand_rank: i64 = 0;
    for (x,_) in hand_capt.iter().enumerate(){
        let key = hand_capt[x];
        if !key.is_empty(){
            match !card_calc.contains_key(key) {

                true => {
                    card_calc.insert(key,1);
        
                }
                false => {
                
                    card_calc.insert(key, 1 + card_calc.get(key).unwrap() );
                }
                     
            }
        }

    };
    let mut highest_val: i64=0;
    for (key, val) in card_calc.iter() {
        
        //println!("key: {key} val: {val}");
        
        let mut card_value: u32 = 0;
        for (x  ,rank) in card_order.iter().enumerate(){
            let tmp_x = x as u32;
            if rank == key{
                card_value = 13 - tmp_x;
            }
            if val>&highest_val{
                highest_val=val.clone()
            }
        }
       
        hand_rank = hand_rank + (val * base.pow(card_value));
        

    }
 
    if highest_val > 3 {
        highest_val +=2;
    }else if highest_val ==3{
        highest_val +=1;
        for (_key, val) in card_calc.iter() {
            let num_match =2;
            if &num_match  == val{
                highest_val +=1;
                
                break;
            }
        }
        
    }else if highest_val ==2{
        let mut counter =0;
        for (_key, val) in card_calc.iter() {
            let num_match =2;
            if &num_match  == val{
                counter +=1;
                
                
            }
        }
        if counter > 1{
            highest_val +=1;
        }
    }
  
    
    //println!("most of one card: {}",highest_val);   
   // hand_rank =hand_rank + highest_val * base.pow(15);
   // println!("handvalue is: {}",hand_rank);
    return highest_val
}



