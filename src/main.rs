use std::io::{self,BufReader,BufRead};
use std::{fs::*, result};
use std::{process, collections::HashMap,error::Error};
#[derive(Debug)]
struct Game{
    game_id:u32,
    content_slice:String
}
impl Game{
    fn build_game(line: &str) -> Result<Game, Box<dyn Error>>{
        let parsed_line = line.split(":").collect::<Vec<&str>>();
        let game_id_string = parsed_line.first().expect("Error accessing the first element");
        let binding = game_id_string.split(" ").collect::<Vec<&str>>();
        let number_string = binding.last().expect("Error accesing the last element");
        let game_id = number_string.parse::<u32>().expect("Error parsing");
        let content_slice = parsed_line.last().expect("Error accesing");
        let content_slice_string = String::from(*content_slice);
        Ok(Game{game_id, content_slice: content_slice_string})
    }
}
fn initialize_buffer() -> Result<BufReader<File>, io::Error>{
    let handler = File::open("assets/problem.txt")?;
    let buffer = BufReader::new(handler);
    Ok(buffer)
}
fn calculate_valid_games(game_list: &[Game], game_rule:&HashMap<&str, u32>) ->u32 {
    let mut result:u32 = 0;
    let mut flag:bool = false;
    for game in game_list{
        let cubes = game.content_slice.split(|c| c == ';' || c == ',').collect::<Vec<&str>>();
        for cube in cubes{
            let trimmed_cube = cube.trim();
            let trimmed_cube_vec = trimmed_cube.split(" ").collect::<Vec<&str>>();
            let key: &&str = trimmed_cube_vec.last().expect("Error");
            let value_string  = trimmed_cube_vec.first().expect("Error");
            let value = value_string.parse::<u32>().expect("Error parsing");
            if let Some(hashmap_value) = game_rule.get(key){
                if value > *hashmap_value{                    
                    flag = false;
                    break;
                }   
            }
            flag = true;
        }
        if flag{
            result+= game.game_id;
        }
        
    }
    result
}
fn calculate_power_of_set(game_list: &[Game]) -> u32{
    let mut result:u32 = 0;
    let mut map :HashMap<&str, u32>= HashMap::new();
    let mut flag:bool = false;
    for game in game_list{
        let cubes = game.content_slice.split(|c| c == ';' || c == ',').collect::<Vec<&str>>();
        for cube in cubes{
            let trimmed_cube = cube.trim();
            let trimmed_cube_vec = trimmed_cube.split(" ").collect::<Vec<&str>>();
            let key: &&str = trimmed_cube_vec.last().expect("Error");
            let value_string  = trimmed_cube_vec.first().expect("Error");
            let value = value_string.parse::<u32>().expect("Error parsing");
            let max_value = map.entry(key).or_insert(value);
            if value > *max_value{
                match map.insert(&key, value) {
                    Some(inserted) => {}
                    other => {}
                }
            }    
        }
        power_result(&mut result, &map);
        map.clear();
    }
    result
}
fn power_result(result: &mut u32, map: &HashMap<&str, u32>)-> (){
    let mut partial_result:u32 = 1;
    for (key, val) in map.iter(){
        partial_result*=(*val);
    }
    *result+=partial_result;
}
fn main() {
    let game_rule= HashMap::from([
        ("red", 12 as u32),
        ("blue", 14 as u32),
        ("green", 13 as u32)
    ]);
    let mut game_list:Vec<Game> = vec![];
    let buffer = initialize_buffer().unwrap_or_else(|err|{
        println!("Problem opening the file: {}", err);
        process::exit(1);
    });
    for row in buffer.lines(){
        if let Ok(line) = row{
            game_list.push(Game::build_game(&line).expect("Error"));
        }
    }
    let result = calculate_power_of_set(&game_list);
    println!("{result}");
}
