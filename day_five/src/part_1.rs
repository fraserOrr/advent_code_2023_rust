use std::fs;
use regex::Regex;


struct Seed {
    number:i64,
    soil:i64,
    fertilizer:i64,
    water:i64,
    light:i64,
    temprature:i64,
    humidity:i64,
    location:i64,
}

struct Mapping {
    dest:i64,
    source:i64,
    range:i64,

}
fn main()  {
    let file = fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    let mut seed_container: Vec<Seed> = Vec::new();
    let mut seed_2_soil: Vec<Mapping> = Vec::new();
    let mut soil_2_fert: Vec<Mapping> = Vec::new();
    let mut fert_2_water: Vec<Mapping> = Vec::new();
    let mut water_2_light: Vec<Mapping> = Vec::new();
    let mut light_2_temp: Vec<Mapping> = Vec::new();
    let mut temp_2_hum: Vec<Mapping> = Vec::new();
    let mut hum_2_loc: Vec<Mapping> = Vec::new();
    //println!("content: {:?}", file);
    let parts_of_content: Vec<&str> = file.split("\r\n").collect();
    let mut count:i64 = 1;
    
    //populate all mappings
    for part in parts_of_content.iter(){
        match part.is_empty() {
            true =>{
                println!("-------- next bit ------");
                count +=1;
            },
            false =>{

                match count {
                    1 => {
                        println!("Seeds");
                        println!("{}",&part);
                        let mut seeds: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            seeds.push(seed_num.parse::<i64>().expect("not number matched"));
                        };

                        for item in seeds.iter(){
                            let mut tmp_seed = Seed{
                                number : item.clone(),
                                soil: 0,
                                fertilizer: 0,
                                water: 0,
                                light: 0,
                                temprature: 0,
                                humidity: 0,
                                location: 0,
                            };
                            seed_container.push(tmp_seed)
                        }

                    },
                    2 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            seed_2_soil.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                        
                       

                    },
                    3 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            soil_2_fert.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                            
                        
                       

                    },
                    4 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            fert_2_water.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                        
                       

                    },
                    5 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            water_2_light.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                       

                    },
                    6 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            light_2_temp.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                       

                    },
                    7 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            temp_2_hum.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                       

                    },
                    8 => {
                        
                        let mut mapping: Vec<i64> = vec![];
                        let re = Regex::new(r"([0-9]{1,10})").unwrap();
                        for (_, [seed_num]) in re.captures_iter(part).map(|c| c.extract()) {
                            mapping.push(seed_num.parse::<i64>().expect("no number matched"));
                        };
                        
                        if mapping.len() > 0 {
                            let mut tmp_mapping = Mapping{
                                dest: mapping[0],
                                source: mapping[1],
                                range: mapping[2],

                            };
                            hum_2_loc.push(tmp_mapping);
                            //println!("mapping: start {} out {} range {}", mapping[0],mapping[1],mapping[2]);

                        }
                        
                       

                    },
                    _ =>{
                        println!("non match")
                    }
                }
                //println!("{part}")
            }
        }
        
    }

    //what do i do with mappings

    for seed in seed_container.iter_mut(){
        //for each seed

        for mapping in seed_2_soil.iter(){
            if seed.number >= mapping.source && seed.number < (mapping.source + mapping.range){
                seed.soil = work_mapping(seed.number, mapping);
                //println!("mapping found for seed {} soil is {}",seed.number,seed.soil);
                break;
            }else{
                seed.soil = seed.number
            }
        }
        //println!("mapping found for seed {} soil is {}",seed.number,seed.soil);
        for mapping in soil_2_fert.iter(){
            if seed.soil >= mapping.source && seed.soil < (mapping.source + mapping.range){
                seed.fertilizer = work_mapping(seed.soil, mapping);
                //println!("mapping found for seed {} soil is {}",seed.number,seed.soil);
                break;
            }else{
                seed.fertilizer = seed.soil
            }
        }
        //println!("mapping found for seed {} fert is {}",seed.number,seed.fertilizer);
        for mapping in fert_2_water.iter(){
            if seed.fertilizer >= mapping.source && seed.fertilizer < (mapping.source + mapping.range){
                seed.water= work_mapping(seed.fertilizer, mapping);
                //println!("mapping found for seed {} fertilizer is {}",seed.number,seed.fertilizer);
                break;
            }else{
                seed.water = seed.fertilizer
            }
        }
        //println!("mapping found for seed {} water is {}",seed.number,seed.water);
        for mapping in water_2_light.iter(){
            if seed.water >= mapping.source && seed.water < (mapping.source + mapping.range){
                seed.light= work_mapping(seed.water, mapping);
                //println!("mapping found for seed {} water is {}",seed.number,seed.water);
                break;
            }else{
                seed.light = seed.water
            }
        }
        //println!("mapping found for seed {} light is {}",seed.number,seed.light);
        for mapping in light_2_temp.iter(){
            if seed.light >= mapping.source && seed.light < (mapping.source + mapping.range){
                seed.temprature= work_mapping(seed.light, mapping);
                //println!("mapping found for seed {} light is {}",seed.number,seed.light);
                break;
            }else{
                seed.temprature = seed.light
            }
        }
        //println!("mapping found for seed {} temp is {}",seed.number,seed.temprature);
        for mapping in temp_2_hum.iter(){
            if seed.temprature >= mapping.source && seed.temprature < (mapping.source + mapping.range){
                seed.humidity= work_mapping(seed.temprature, mapping);
                //println!("mapping found for seed {} temprature is {}",seed.number,seed.temprature);
                break;
            }else{
                seed.humidity = seed.temprature
            }
        }
        //println!("mapping found for seed {} hum is {}",seed.number,seed.humidity);
        for mapping in hum_2_loc.iter(){
            if seed.humidity >= mapping.source && seed.humidity < (mapping.source + mapping.range){
                seed.location= work_mapping(seed.humidity, mapping);
                //println!("mapping found for seed {} humidity is {}",seed.number,seed.humidity);
                break;
            }else{
                seed.location = seed.humidity
            }
        }
         //println!("mapping found for seed {} location is {}",seed.number,seed.location);
    }

    let mut lowest:i64 = 0;

    for seed in seed_container.iter(){
        if lowest == 0{
            lowest = seed.location
        }
        if seed.location < lowest{
            lowest = seed.location
        }
    }

    println!("lowest: {lowest}")
}

fn work_mapping  (num:i64,mapping:&Mapping) ->i64{
    let diff =mapping.dest - mapping.source;
    //println!("{diff} : ");
    match diff >= 0 {
        true =>{
            
            return num + diff 
        },
        false => {
            match num + diff < 0{
              true =>{
                0
              },
              false => {
                return num + diff
              }
            }
            
        }
    }
    
}