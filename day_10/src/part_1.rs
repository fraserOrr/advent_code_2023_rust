use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use array2d::{Array2D, Error};


struct Node{
    x: usize,
    y: usize,
    content: char,

}
fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("./src/input.txt")?;
    let  buf_reader = BufReader::new(file);
    let m_x = 140;
    let m_y = 140;

    let mut char_map = Array2D::filled_with('.',m_x,m_y);
    let mut start: Node = Node{
        x: 0,
        y: 0,
        content: 'b',
    };
    let mut path: Vec<Node> = vec![];

    for (y,line) in buf_reader.lines().enumerate() {

        
    	let  line_content = line.expect("oops file error");
        for (x,char_loaded) in line_content.chars().enumerate(){
            let result = char_map.set(y,x,char_loaded);
            assert_eq!(result, Ok(()));
            if char_loaded == 'S'{
                start.x = x;
                start.y = y;
                start.content = 'S';
            }
        }
        //println!();
    }
    if start.content == 'b'{
        panic!("start not found");
    }else{

        
        path.push(start);
    }
    
    println!("start at y: {} x: {}",path.get(0).expect("no starting value").y,path.get(0).expect("no starting value").x);
    let mut looped=false;
    //determine intial vector

    let mut dir = start_move(&char_map,&Node{x:path.last().unwrap().x,y: path.last().unwrap().y,content:path.last().unwrap().content});
    
    let mut next_node: Node;
    while looped!=true{
        let current_node= Node{x:path.last().unwrap().x,y: path.last().unwrap().y,content:path.last().unwrap().content};
        println!("current {} coming from {dir}",current_node.content);
        match dir{
            'U' => {
               next_node = Node{
                   x: current_node.x,
                   y: current_node.y-1,
                   content: char_map.get(current_node.y-1,current_node.x).expect("char ref error").clone(),
               }  
            },
            'D' =>{
               next_node = Node{
                   x: current_node.x,
                   y: current_node.y+1,
                   content: char_map.get(current_node.y+1,current_node.x).expect("char ref error").clone(),
               }                 
            },
            'L'=>{
               next_node = Node{
                   x: current_node.x-1,
                   y: current_node.y,
                   content: char_map.get(current_node.y,current_node.x-1).expect("char ref error").clone(),
               } 
               
            },
            'R' =>{
               next_node = Node{
                   x: current_node.x+1,
                   y: current_node.y,
                   content: char_map.get(current_node.y,current_node.x+1).expect("char ref error").clone(),
               } 
                

            },
            _ =>{
                panic!("returner failed")
            }
        }
        match next_node.content{
            '|' =>{
                if dir == 'D'{
                    dir = 'D'
                }else{
                    dir = 'U'
                }
            },
            '-' =>{
                if dir == 'L'{
                    dir = 'L'
                }else{
                    dir = 'R'
                }
            },
            'F' =>{
                if dir == 'U'{
                    dir = 'R'
                }else{
                    dir = 'D'
                }
            },
            'L' =>{
                if dir == 'D'{
                    dir = 'R'
                }else{
                    dir = 'U'
                }
            },
            '7' =>{
                if dir == 'R'{
                    dir = 'D'
                }else{
                    dir = 'L'
                }
            },
            'J' =>{
                if dir == 'R'{
                    dir = 'U'
                }else{
                    dir = 'L'
                }
            },
            _ =>{
            
            },
        }

        path.push(Node{x:next_node.x,y: next_node.y,content:next_node.content});

        

        if next_node.content=='S'{
            looped=true;
        }
    }

    for node in path.iter(){
        println!("{}",node.content)
    }

    println!("length of compelte path: {}",path.len());
    Ok(())

}

fn start_move(map: &Array2D<char>, current_node: &Node) -> char{
    let up_valid_move: Vec<char>= vec!['|','F','7'];
    let down_valid_move: Vec<char>= vec!['|','J','L'];
    let right_valid_move: Vec<char>= vec!['-','J','7'];
    let left_valid_move: Vec<char>= vec!['|','F','L'];

    let move_dir: Vec<char> = vec!['U','D','L','R'];
    let mut comparison_node: Node;
    for dir in move_dir.iter(){
        match dir {
            'U' => {
                if current_node.y != 0{
                    let comparison_content = map.get(current_node.y-1,current_node.x).expect("char ref error").clone();

                    if up_valid_move.contains(&comparison_content){
                      return dir.clone()
                    }
                    

                }
            },
            'D' =>{
                if current_node.y != map.num_rows()-1{
                    let comparison_content = map.get(current_node.y+1,current_node.x).expect("char ref error").clone();

                    if down_valid_move.contains(&comparison_content){
                        return dir.clone()
                    }
                    

                }
            },
            'L'=>{
                if current_node.x != 0{
                    let comparison_content = map.get(current_node.y,current_node.x-1).expect("char ref error").clone();

                    if left_valid_move.contains(&comparison_content){
                        return dir.clone()
                    }
                    

                }
            },
            'R' =>{
                if current_node.x != map.num_columns()-1{
                    let comparison_content = map.get(current_node.y,current_node.x+1).expect("char ref error").clone();

                    if right_valid_move.contains(&comparison_content){
                        return dir.clone()
                          
                    }
                    

                }

            },
            _ =>{
                return 'B'
            }

        }

    }
    
    panic!("couldnt find valid starting move");
    
}
