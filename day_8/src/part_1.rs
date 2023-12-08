use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use num::integer::lcm;

struct Node {
    start: String,
    left: String,
    right: String,
}

fn main() {
    // File hosts.txt must exist in the current path
    let re = Regex::new(r"([A-Z]{3})").unwrap();
    let mut instructions: Vec<String> = vec![];
    let mut map: Vec<Node> = vec![];
    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (x,line) in lines.enumerate() {
            if let Ok(ip) = line {
               let thing = ip.clone();
                match x {
                    0 => {
                         println!("Get intructs");
                        
                         instructions = ip.split("").filter(|&x| !x.is_empty()).map(|m| m.to_string()).collect();
                         
                    },
                    2..=1000 =>{

                        let node_mappings: Vec<_> = re.find_iter(&thing).map(|m| m.as_str()).collect();
                        
                        let tmp = Node{
                            start: node_mappings[0].to_string(),
                            left: node_mappings[1].to_string(),
                            right: node_mappings[2].to_string(),
                        };
                        map.push(tmp);
                    },
                    _ => {
                        println!("empty line");
                    }
                }

            }
        }
    }

    part_1(&instructions,&map);
    println!("-----");
    println!("{}",part_2(&instructions,&map));
   
    // nice

   
}
fn part_2(instructions: &Vec<String>,map: &Vec<Node>) -> usize{
     let starting_nodes = map.iter().filter(|node| node.start.ends_with("A"));
     starting_nodes
        .map(|node| {
            let mut step_counter = 0;
            let mut current_node = node;
            while !current_node.start.ends_with("Z") {
                let next_id = if instructions[step_counter % instructions.len()] == "L"{
                    &current_node.left
                } else {
                    &current_node.right
                };
                current_node = map.iter().find(|node| node.start == *next_id).unwrap();
                step_counter += 1;
            }
            step_counter
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap()


}






fn part_1(instructions: &Vec<String>,map: &Vec<Node>){
    let mut cur_move: String;
    let mut move_counter: usize = 0;
    
 
    let mut  current_map  = map.iter().find(|node| node.start == "AAA").unwrap();
    while current_map.start!="ZZZ" {
       
       let current_intruct = instructions.get(move_counter % instructions.len()).unwrap();
       
       if current_intruct.to_string() =="L"{
           cur_move=current_map.left.clone();

       }else if current_intruct.to_string() =="R"{ 
           cur_move=current_map.right.clone();       
       }else{
           println!("error finding move");
           cur_move="Fail".to_string();
       }; 
       move_counter +=1;
       current_map  = map.iter().find(|node| node.start == cur_move).unwrap();    

    }

    println!("Made it in {} moves", move_counter);

}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}