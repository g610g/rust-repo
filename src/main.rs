use std::io::{self, Read};
use std::fs::File;
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
    let mut point_generic = Point{x:vec![1,2,3,4], y:1};
    let generic_field = point_generic.x();
    let user = practice_package::authentication::User::new(String::from("g6i10"), String::from("password"));
    practice_package::authentication::verify_email(&user);
    user.display_user_credentials();
    println!("{:?}", point_generic.x);
    // println!("{}", );
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let a = 32;
    let b = 1;
    compare_two_int(a, &b);
    println!("{secret_number}");
    println!("Welcome to our guessing game program!");
    println!("Enter a guess between 1-100");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error reading the stdin");
    let guess:u32 = guess.trim().parse().expect("Error parsing value");
    let guess_struct = Guess::new(guess);
  

}
fn read_file() -> Result<String, io::Error>{
    let mut username_file_handle = File::open("Hello.txt")?;
    let mut username = String::new();
    username_file_handle.read_to_string(&mut username)?;
    Ok(username)
}
fn compare_two_int(a:u32, b:&u32){
    match a.cmp(b){
        Ordering::Less => {println!("less")}
        Ordering::Equal => {println!("Equal")}
        Ordering::Greater => {println!("Greater")}
    }
}