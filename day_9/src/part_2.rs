use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

//losing neg num?

fn main() {
    let mut total=0;
    let re = Regex::new(r"([-0-9]+)").unwrap();

    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (x,line) in lines.enumerate() {
            if let Ok(ip) = line {
                println!();
                println!("line: {}, value: {}",x,ip);
                let entry_values: Vec<_> = re.find_iter(&ip).map(|x|  x.as_str().to_string().parse::<i64>().expect("not num")).collect();              
                let mut container: Vec<Vec<i64>>= vec![];
                container.push(entry_values);
                let mut done = false;

                while done==false{
                    let curr_sequence = container.last().expect("missing vector");
                    done=true;
                    for x in 0..(curr_sequence.len()){
                        if container.last().expect("missing vector").get(x).expect("bad sub vecotr ref {x}").to_string().parse::<i64>().expect("not num") !=  0 {
                            done=false;  
                        }
                    }
                    if done ==false {
                        container.push(sequence_diff( container.last().expect("missing vector") ));
                    }
                }

                for sequence in container.iter(){
                
                    println!("{:?}", sequence);
                }
                 println!();
                total += give_estimate(container);

                
            }

          
        }
        println!("Total: {total}");
    }

 
}
fn give_estimate(mut container: Vec<Vec<i64>>) -> i64{
    for x in (1..container.len()-1).rev(){
        let new_value =  container.get(x-1).expect("no container").get(0).expect("no num") - container.get(x).expect("no container").get(0).expect("no num");
        
        container.get_mut(x-1).expect("no container").insert(0,new_value);

    }
    for sequence in container.iter(){
                
        println!("{:?}", sequence);
    }
    return container.get(0).expect("no container").get(0).expect("no num").clone()
}

fn sequence_diff(curr_sequence: &Vec<i64>) -> Vec<i64>{
    let mut return_vector: Vec<i64> = vec![];
    for (x,item) in curr_sequence.iter().enumerate(){
        if x >= curr_sequence.len()-1{
            break;
        }
        if curr_sequence.len() == 1{
            println!("{:?}", curr_sequence);
            let diff = item.clone();
            return_vector.push(diff);
        }else{
            let diff =  curr_sequence[x+1] - item;
            return_vector.push(diff);
        }
        
        
    }

    return return_vector;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}