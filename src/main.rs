use std::io::{self,BufReader,BufRead, Error};
use std::fs::*;
use std::{process};
struct Game<'a >{
    game_id:u32,
    content:Vec<&'a str>
}
fn initialize_buffer() -> Result<BufReader<File>, io::Error>{
    let handler = File::open("assets/example.txt")?;
    let buffer = BufReader::new(handler);
    Ok(buffer)
}
fn filter_line<'a >(line: &'a str)-> Game{
    let sub_string:Vec<&str> = line.split(":").collect();
    let binding = sub_string.first().expect("Error splitting the string")
                        .split(" ").collect::<Vec<&str>>();
    let game_id = binding.last().expect("Error accesing the element");
    let content_vector = sub_string.last().expect("Error Accesing");
    let content:Vec<&str> = content_vector.split(";").collect();
    let game_id:u32 = game_id.parse::<u32>().expect("Error converting the id into u32");
    Game{game_id, content}
}
fn main() {
    let mut game_list:Vec<Game> = vec![];
    let contents = String::new();
    let buffer = initialize_buffer().unwrap_or_else(|err|{
        println!("Problem opening the file: {}", err);
        process::exit(1);
    });
    for row in buffer.lines(){
        if let Ok(line) = row{
            game_list.push(filter_line(&line));

        }
    }    
}
