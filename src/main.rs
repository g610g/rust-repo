use std::io::{self,BufReader,Read,BufRead};
use std::fs::*;
use std::cmp::{Ord,Ordering};
use std::collections::HashMap;
use rand::Rng;
use practice_package::authentication::*;
struct Guess{
   value:u32
}
struct Point <T, U>{
    x:T,
    y:U
}
impl <T, U> Point<T, U>{
    fn x(&self) -> &T{
        &self.x
    }
}
impl Guess{
    fn new(value:u32) -> Guess{
        if value < 1 || value > 100{
            panic!("Value is not between 1 and 100");
        }
        Guess{value}
    }
    fn get_value(&self) -> u32{
        self.value
    }
}
fn main() {
    let a = String::from("abcdef");
    let c = life_times_test(&a);
    // read_file("src/test.txt");    
}
fn read_file(path: &str) ->(){
    let handle = File::open(path).unwrap();
    let mut buffer =  BufReader::new(handle);
    let mut contents = String::new();
//   for lines in buffer.lines(){
//     println!("{}", lines.unwrap());
//   }
    if let Ok(len) = buffer.read_line(&mut contents){
        println!("len:{}", len);
    }
    println!("{}", contents);
}
fn longest<'a, 'b> (a: &'a str, b :&'b str) -> &'b str{
    if a.len() > b.len(){
        a
    }else{
        b
    }
}
fn life_times_test(x: &str)-> &str{
    let b = String::from("bcd");
    longest(x, &b)
}