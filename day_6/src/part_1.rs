use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;



fn main() {
    // File hosts.txt must exist in the current path
    let re = Regex::new(r"([0-9]+)").unwrap();
    let mut time: Vec<i64> = vec![];
    let mut distance: Vec<i64> = vec![];
    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (x,line) in lines.enumerate() {
            if let Ok(ip) = line {
                println!("{}", ip);

                match x {
                    0 => {
                        for (_, [count]) in re.captures_iter(&ip).map(|c| c.extract()) {
                            time.push(count.parse::<i64>().expect("missed number"));
                        }
                    },
                    1 => {
                        for (_, [count]) in re.captures_iter(&ip).map(|c| c.extract()) {
                            distance.push(count.parse::<i64>().expect("missed number"));
                        }
                    },
                    _ =>{
                        println!("option not found")
                    }


                }

            }
        }
    }
    let mut total = 1;
    for (x,_) in time.iter().enumerate(){
        let mut race_combo = 0;
        let tmp_time=  time[x];
        let tmp_distance = distance[x];
        println!("Race {x} had time {} for distance {}",tmp_time,tmp_distance);
        println!("----------");
        for time in 1..tmp_time{
            let speed = time;
            let distance_travelled = speed * (tmp_time- time);
            println!("for time held {time} the boat traveled {distance_travelled}mm in {}ms at {}mmph",tmp_time,speed);

            if distance_travelled > tmp_distance{
                race_combo +=1;
            };
        };
        println!("----------");
        println!("race combo: {race_combo}");
        println!("----------");
        total *= race_combo;
    }
    println!("total {total}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}