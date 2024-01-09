use std::io;
#[derive(Debug)]
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
#[derive(Debug)]
struct Dog{
    name:String,
    age:u8,
    breed_type:String,
    color:String,
    height: f32,
    is_alive:bool
    
}
#[derive(Debug)]
enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message{
    fn call(&self){
        dbg!(&self);
    }
}
impl Dog{
    fn bark(&self){
        println!("Woof!");
    }
    fn say_name(&self){
        println!("My name is {}", self.name);
    }
    fn create_dog(name:String, age:u8, breed_type:String, color:String, height:f32, is_alive:bool) -> Self{
        Self{
            name,
            age,
            breed_type,
            color,
            height,
            is_alive
        }
    }
}
fn main() {
    
    let name = "Gio Gonzales";
    //tuple struct
    struct Box(u32, u32);
    let my_box = Box(15, 17);
    // println!("{:?}", my_box);
    // let mut user = User{
    //     username:String::from("g6i1o0"),
    //     active:true,
    //     email:String::from("gio.gonzales@carsu.edu.ph"),
    //     sign_in_count:1
    // };
    // let hotdog = Dog{
    //     name: String::from("Hotdog"),
    //     age:2,
    //     breed_type:String::from("Duchsand"),
    //     color:String::from("brown"),
    //     height:12.5,
    //     is_alive:true
    // };
    // let cookie = create_dog(hotdog, String::from("Cookie"));
    // cookie.bark();
    // cookie.say_name();
    // let eggsy = Dog::create_dog(String::from("Eggsy"), 3, String::from("Aspin"), String::from("black"), 5.13, true);
    // let mut user_input  = String::new();
    // let bytes_of_result = io::stdin().read_line(&mut user_input)
    //            .expect("Error reading the input stream!");
    // println!("The user input: {}", user_input);
    // println!("Bytes: {}", bytes_of_result);
    let my_str = "()())";
    // println!("result: {}", not_quite_lisp(&my_str));
    println!("position: {}",note_quite_lisp_2(&my_str));
    let message = Message::ChangeColor(1,2,3);
    message.call();
    fn string_slice(s1: &str) -> &str{
        let bytes = s1.as_bytes();
        for (index, &character) in bytes.iter().enumerate(){
            if character == b' '{
                return &s1[..index];
            }
        }
        &s1[..]
}
}
fn build_user(user:User)-> User{
    User{
        email:String::from("test@gmail.com"),
        ..user
    }
}
fn steal_struct(user: User){
    println!("{:?}", user);
}
fn test_number(num:u32){
    println!("{num}");
}
fn create_dog(origin_dog: Dog, name: String) -> Dog{
    Dog{
        name,
        breed_type:String::from("Duchsand"),
        color:String::from("brown"),
        ..origin_dog
    }
}
fn not_quite_lisp(my_string: &str) -> i64{
    let mut result: i64 = 0;
    let increment_floor = '(';
    let decrement_floor = ')';
    for c in my_string.chars(){
        if c == increment_floor{
            result+=1;
        }else if c == decrement_floor{
            result-=1;
        }
    }
    result
}
fn note_quite_lisp_2(pattern: &str) -> u64{
    let mut position: u64 = 0;
    let mut result: i64 = 0;
    let increment_floor = '(';
    let decrement_floor = ')';
    for c in pattern.chars(){
        position+=1;
        if c == increment_floor{
            result+=1;
        }else if c == decrement_floor{
            result-=1;
        }
        if result == -1{
            return position;
        }
    }
    position
}