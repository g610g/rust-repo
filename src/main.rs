use std::io::{self,BufReader,BufRead};
use std::fs::*;
#[derive(Debug)]
enum CustomResult<T>{
    Tup(T),
    Digit(u32),
    Null(Option<T>)
}
#[derive(Debug)]
struct CustomTup<'a>{
    text: &'a str,
    index:usize
}
fn find_first_number<'a >(vec: &Vec<&str>, line:&'a str) -> CustomResult<CustomTup<'a>>{
    let mut min_index:u32 = 0;
    let mut result:CustomResult<CustomTup> = CustomResult::Null(None);
    //find the first word if there is any
    for (num_index, num) in vec.iter().enumerate(){
        if let Some(tup) = line.match_indices(num).collect::<Vec<(usize, &str)>>().get(0){
            let (index, string) = tup;
            if (min_index != 0 && *index + 1 < min_index.try_into().unwrap()) || min_index == 0{
                min_index = *index as u32 + 1;
                result = CustomResult::Tup(CustomTup{text:string, index:num_index});
            }
        }
    }
    if min_index == 0{
        min_index = line.len() as u32;
    }
    for (char_index, character) in line.chars().enumerate(){
        
        if character.is_numeric() && char_index + 1 <= min_index.try_into().unwrap(){
            min_index = char_index as u32 + 1;
            if let Some(digit) = character.to_digit(10){
                result = CustomResult::Digit(digit);   
            }
        }
    }
    // println!("{:?}", result);
    result
}
fn find_second_number<'a >(vec: &Vec<&str>, line:&'a str) -> CustomResult<CustomTup<'a>>{
    let mut max_index:u32 = 0;
    let mut result:CustomResult<CustomTup> = CustomResult::Null(None);
    //find the first word if there is any
    for (num_index, num) in vec.iter().enumerate(){
        if let Some(tup) = line.match_indices(num).collect::<Vec<(usize, &str)>>().last(){
            let (index, string) = tup;
            if *index > max_index.try_into().unwrap(){
                max_index = *index as u32;
                result = CustomResult::Tup(CustomTup{text:string, index:num_index});
            }
        }
    }
    for (char_index, character) in line.chars().enumerate(){
        
        if character.is_numeric() && char_index >= max_index.try_into().unwrap(){
            max_index = char_index as u32;
            if let Some(digit) = character.to_digit(10){
                result = CustomResult::Digit(digit);   
            }
        }
    }
    // println!("{:?}", result);
    result
}
fn main() {
    let list = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]; 
    let mut first = 0;
    let mut second = 0;
    let mut result = 0;
    let handler = File::open("assets/problem.txt").expect("Error handling the file");
    let buffer_reader = BufReader::new(handler);
    for row in buffer_reader.lines(){
        if let Ok(text) = row {
            match find_first_number(&list, &text){
                CustomResult::Tup(result) => {
                    first = result.index as u32;
                }
                CustomResult::Digit(num) => {
                    first = num;
                }
                other => {}
            };
            match find_second_number(&list, &text){
                CustomResult::Tup(result) => {
                    second = result.index as u32
                }
                CustomResult::Digit(num) => {
                    second = num
                }
                other => {}
            }            
            let concat_string = format!("{}{}", first, second);
            // println!("{concat_string}");
            result+=concat_string.parse::<u32>().unwrap();
        }
    }
    println!("{result}");
}
