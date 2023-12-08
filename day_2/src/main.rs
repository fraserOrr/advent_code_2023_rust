use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
    let mut line_counter = 0;
    let mut total = 0;

    for line in buf_reader.lines() {
        //each line represents 1 gaem

        let mut m_red = 0;
        let mut m_blue = 0;
        let mut m_green=0;
        line_counter = line_counter + 1;
        
    	let content = line.expect("oops file error");
    	//print_string( &content);
    	let mut beads: Vec<(i32, &str)> = vec![];

    	let re = Regex::new(r"([0-9]{1,2})( red| blue| green)").unwrap();

        for (_, [count, colour]) in re.captures_iter(&content).map(|c| c.extract()) {
            beads.push((count.parse::<i32>()?, colour));
        }

        //println!("initial vector: {:?}", beads);
        //let mut line_accepted = true;
        for item in beads.iter() {
            
            //item.0 is the number of beads item.1 is the colour i just need to keep a list of them
            println!("we have {} beads of colour {}", item.0, item.1);
            // we need to comapare colou adn value to max

            if item.1 == " red" {
              if item.0 > m_red{
                  m_red = item.0;
              };
            };
            if item.1 == " green" {
              if item.0 > m_green{
                  m_green = item.0;
              };
            };
            if item.1 == " blue" {
              if item.0 > m_blue{
                  m_blue = item.0;
              };
            };

            
        }
        let game_total = m_red * m_green * m_blue ;
        total = total + game_total;

        println!("for game {line_counter} the max for each bead colour was red {m_red} blue {m_blue} green {m_green}"    );
        

        

    }

    println!("total: {}", total);
    Ok(())

}
