use std::io::{self,BufReader,BufRead};
use std::fs::*;
fn main() {

    let mut first:u32 = 0;
    let mut second:u32 = 0;
    let mut result = 0;
    let handler = File::open("assets/problem.txt").expect("Error handling the file");
    let buffer_reader = BufReader::new(handler);
    for row in buffer_reader.lines(){
        if let Ok(text) = row {
            for (index, character) in text.chars().enumerate(){
                if character.is_numeric(){
                    if let Some(digit) = character.to_digit(10){
                        if first == 0 {
                            first = digit;
                        }else {
                            second = digit;
                        }
                    }
                }                   
            }
            if second == 0{
                second = first;
            }
            let concat_string = format!("{}{}", first, second);
            result+=concat_string.parse::<u32>().unwrap();
            first = 0;
            second = 0;
        }

    }
    println!("{result}");
}