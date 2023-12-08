use std::fs;
use regex::Regex;

struct Mappings {
    dest_start: i128,
    src_start: i128,
    length: i128,
}


fn main() {

    let mut counter: i128 = 0;

    //Input Parsing

    let path = "./src/input2.txt";
    let input = fs::read_to_string(path).expect("Error Reading File");

    let seed_pattern = Regex::new("seeds:(?<seeds>.*)").unwrap();

    let Some(seed_input) = seed_pattern.captures(&input) else {
        println!("no match!");
        return;
    };

    let mapping_pattern = Regex::new(r"\S+ \w+:\n").unwrap();

    let mut mapping_input = mapping_pattern.split(&input);

    mapping_input.next().unwrap().to_string();

    //Arrays/Vectors
    let seeds = seed_input.name("seeds").unwrap().as_str().split_ascii_whitespace();
    let seed_iter: Vec<i128>  = seeds.map(|x| x.parse::<i128>().unwrap()).collect();

    let seed_to_soil_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let soil_to_fertilizer_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let fertilizer_to_water_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let water_to_light_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let light_to_temp_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let temp_to_humidity_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    let humidity_to_location_input: Vec<Mappings> = create_mapping(mapping_input.next().unwrap().to_string());
    //End Arrays/Vectors

    let mut lowest: i128 = 9223372036854775807;

    for i in (0..seed_iter.len()-2).step_by(2){

        let value = seed_iter[i];
        let range = seed_iter[i + 1];

        let test = value + range;

        println!("Start = {}, Range = {}, Total = {}", value, range, test);

        for seed in value..value + range{
            //Per Seed
            let soil = create_output(seed, &seed_to_soil_input);
            let fertilizer = create_output(soil, &soil_to_fertilizer_input);
            let water = create_output(fertilizer, &fertilizer_to_water_input);
            let light = create_output(water, &water_to_light_input);
            let temperature = create_output(light, &light_to_temp_input);
            let humidity = create_output(temperature, &temp_to_humidity_input);
            let location = create_output(humidity, &humidity_to_location_input);
            
            if location < lowest {
                lowest = location;
            }

            counter += 1;
            if counter % 1000000 == 0{
                println!("Seeds checked: {}, Current Lowest: {}", counter, lowest);
            }
        }
    }

    println!("Lowest Location {}", lowest);
}

fn create_output (input: i128, map: &Vec<Mappings>) -> i128 {

        let mut output = input;

        for item in map{
            let dest_start = item.dest_start;
            let src_start = item.src_start;
            let range = item.length;

            if input < src_start { continue; }
            if input >= src_start + range { continue; }

            let difference = input - src_start;

            output = dest_start + difference;
            return output;
        }

    return output;
}

fn create_mapping(input: String) -> Vec<Mappings> {
    let mut temp_mapping: Vec<Mappings> = Vec::new();
    let lines = input.split("\n");
    
    for line in lines{
        if line.is_empty(){
            continue;
        }
        let mut split_string = line.split_ascii_whitespace();
        let mut new_mapping: Mappings = Mappings { dest_start: 0, src_start: 0, length: 0 };

        new_mapping.dest_start = split_string.next().unwrap().parse().unwrap();
        new_mapping.src_start = split_string.next().unwrap().parse().unwrap();
        new_mapping.length = split_string.next().unwrap().parse().unwrap();

        temp_mapping.push(new_mapping);
    }

    return temp_mapping;
}

