
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;


struct Hand{
    card: String,
    hand_value: i64,
    bet: i64,
    rank: i64,
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    let card_strength: Vec<&str> = Vec::from(["A","K","Q","J","T","9","8","7","6","5","4","3","2"]);

    let mut hand_container: Vec<Hand> = vec![];
    for (count,line) in buf_reader.lines().enumerate() {
       //println!("Hand: {}",count);
       let content = line.expect("oops file error");
       let tmp: Vec<&str>  = content.split(" ").collect();
       let tmp_hand = Hand{
           card: tmp[0].parse::<String>().expect("bad card string"),
           hand_value: calculate_hand(tmp[0].parse::<String>().expect("bad card string"),&card_strength),
           bet: tmp[1].parse::<i64>().expect("bad bet"),
           rank: 0,
       };
       hand_container.push(tmp_hand);
    }

    let tmp = hand_container.get_mut(0).unwrap().hand_value;
    let digits: Vec<_> = tmp.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    
    for k in 1..8{

        for j in (0..14).rev(){
            //let counter = digits.len() - j;
            loop{
                let mut swapped=false;
                for x in 0..hand_container.len()-1{
                    
                    let tmp = hand_container.get_mut(x).unwrap().hand_value;
                    let digits: Vec<u32> = tmp.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

                    let tmp2 = hand_container.get_mut(x+1).unwrap().hand_value;
                    let digits2: Vec<u32> = tmp2.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

                    if digits[j] >=k{
                        if digits[j] > digits2[j]{
                            //println!("swapping {} with {}",digits[j],digits2[j]);
                            hand_container.swap(x,x+1);
                            swapped = true;
                        }
                        
                    }

                }
                if!swapped{
                    break;
                }
            }

            }
    

    }


    let mut total = 0;
    for (x,hand) in hand_container.iter().enumerate().rev(){
        let i = x as i64;
        
        let rank: i64 = i +1 ;
        let score = hand.bet * rank;
        total= total + score;
        println!("in rank {:0>2} with hand: {}  --> {} they placed: {:0>4} and scored: {}",rank,hand.card,hand.hand_value,hand.bet,score);
    }
    
    //249661593
    println!("total: {}",total);
    Ok(())
}

fn calculate_hand(hand : String,card_order: &Vec<&str>) ->i64{
    let mut card_calc: HashMap<&str,i64> = HashMap::new();
    
    let hand_capt: Vec<&str>  = hand.split("").collect();
    let base: i64 = 10;
    let mut hand_value: i64 = 0;
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
        let val_2 = val.clone() as u32;
        let cval_2 = card_value as i64;
        hand_value = hand_value + (val * base.pow(card_value));
        //hand_value = hand_value + (1*cval_2 * base.pow(11+ val_2));

    }
 
    if highest_val > 3 {
        highest_val +=2;
    }else if highest_val ==3{
        highest_val +=1;
        for (key, val) in card_calc.iter() {
            let num_match =2;
            if &num_match  == val{
                highest_val +=1;
                
                break;
            }
        }
        
    }else if highest_val ==2{
        let mut counter =0;
        for (key, val) in card_calc.iter() {
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
    hand_value =hand_value + highest_val * base.pow(14);
   // println!("handvalue is: {}",hand_value);
    return hand_value
}



