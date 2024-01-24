use std::io::{BufReader,BufRead, Error };
use std::process;
use std::fs::{File};
#[derive(Debug)]
struct Human{
    name: String,
    age:i32
}
fn initialize_file(file_path: &str) -> Result<BufReader<File>, Error >{
    let handler = File::open(file_path)?;
    let buffer = BufReader::new(handler);
    Ok(buffer)
}
fn returns_option() -> Option<i32> {
    None
}
fn rename_humans(human_list: &Vec<Human>) -> Vec<Human> {
    human_list.iter().map(|h|{
        return Human{name:format!("{} Gonzales", h.name ), age:h.age + 1}
    }).collect()
}
fn main() {
    // let file_path = "assets/problem.txt";
    // let file_buffer: BufReader<File>; 
    // match initialize_file(file_path){
    //     Ok(buffer) => file_buffer = buffer,
    //     Err(err) => { 
    //         println!("An error as occured: {}", err);
    //         process::exit(1);
    //     }
    // }
    // for line in file_buffer.lines() {
    //     if let Ok(string_line) = line{
    //     }
    // }
    let mut human_list = vec![
        Human{name:"Gio".to_string(), age:22},
        Human{name:"Mary".to_string(), age:15}
    ];
    let human_iter = human_list.iter(); 
    
    let modified_humans:Vec<Human> = human_iter.map(|h| {
        return  Human{name:h.name.clone(), age:h.age + 1};
    }).collect();
    println!("{:?}", rename_humans(&human_list));
    println!("{:?}", human_list);
}