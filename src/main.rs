use std::io::{self,BufReader,BufRead};
use std::fs::*;
use std::{process, collections::HashMap,error::Error};
#[derive(Debug)]
struct Game{
    game_id:u32,
    content_slice:String
}
impl Game{
    fn build_game(line: &str) -> Result<Game, Box<dyn Error>>{
        let sub_string:Vec<&str> = line.split(":").collect();
        let binding = sub_string.first().expect("Error splitting the string")
                            .split(" ").collect::<Vec<&str>>();
        let game_id = binding.last().expect("Error accesing the element");
        let content_vector = sub_string.last().expect("Error Accesing");
        let content:Vec<&str> = content_vector.split(";").collect();
        let content_slice:String = content.join(";");
        let game_id:u32 = game_id.parse::<u32>().expect("Error converting the id into u32");
        Ok(Game{game_id, content_slice})
    }
}
fn initialize_buffer() -> Result<BufReader<File>, io::Error>{
    let handler = File::open("assets/example.txt")?;
    let buffer = BufReader::new(handler);
    Ok(buffer)
}
fn calculate_valid_games(game_list: &[Game], game_rule:&HashMap<&str, u32>) ->u32 {
    let result:u32 = 0;
    for game in game_list{
        // println!("{}", game.content_slice);
        let row:Vec<&str> = game.content_slice.split(";").collect();
        for set in &row{
            println!("{}", set.trim());
        }
        println!("{:?}", row);
    }
    result
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
    calculate_valid_games(&game_list, &game_rule);
    // println!("{:?}", game_list);
}
