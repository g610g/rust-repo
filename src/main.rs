// use std::io;
mod enums;
// use crate::enums::{Circle, Square, Number, Shape};

fn main() {
    
    let name = "Gio Gonzales";
    //tuple struct
    // struct Box(u32, u32);
    // let my_box = Box(15, 17);
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
    // let my_str = "()())";
    // println!("result: {}", not_quite_lisp(&my_str));
    // println!("position: {}",note_quite_lisp_2(&my_str));
    // let message = Message::ChangeColor(1,2,3);
//     let square = enums::Square{
//         sides:24
//     };
//     let circle = enums::Circle{
//         radius:2.14
//     };
//     let my_shape = enums::Shape::Circle(circle);
//     match my_shape.determine_area() {
//         enums::Number::Integer(value) => {println!("{}", value)}
//         enums::Number::Float(value) => {println!("{}", value)}
//     }
//     fn string_slice(s1: &str) -> &str{
//         let bytes = s1.as_bytes();
//         for (index, &character) in bytes.iter().enumerate(){
//             if character == b' '{
//                 return &s1[..index];
//             }
//         }
//         &s1[..]
// }
    let mut my_vec = vec![1,2,3,4,5];
    read_vector(&my_vec);
    let first_value = my_vec[2];
    println!("{first_value}");

    for i in &my_vec{
        println!("{}", *i);
    }
    // match first_value{
    //     Some(i) => {println!("We have a value in the index value: {}", *i);}
    //     None  => {println!("No Value is here");}
    // }
    // for i in &my_vec{
    //     println!("{}", *i);
    // }
    // my_vec.push(3);
    // println!("{}", *first_value);

}
fn read_vector(slice: &[usize]){
    for i in slice{
        println!("{i}");
    }
}
// fn build_user(user:User)-> User{
//     User{
//         email:String::from("test@gmail.com"),
//         ..user
//     }
// }
// fn steal_struct(user: User){
//     println!("{:?}", user);
// }
// fn test_number(num:u32){
//     println!("{num}");
// }
// fn create_dog(origin_dog: Dog, name: String) -> Dog{
//     Dog{
//         name,
//         breed_type:String::from("Duchsand"),
//         color:String::from("brown"),
//         ..origin_dog
//     }
// }
// fn not_quite_lisp(my_string: &str) -> i64{
//     let mut result: i64 = 0;
//     let increment_floor = '(';
//     let decrement_floor = ')';
//     for c in my_string.chars(){
//         if c == increment_floor{
//             result+=1;
//         }else if c == decrement_floor{
//             result-=1;
//         }
//     }
//     result
// }
// fn note_quite_lisp_2(pattern: &str) -> u64{
//     let mut position: u64 = 0;
//     let mut result: i64 = 0;
//     let increment_floor = '(';
//     let decrement_floor = ')';
//     for c in pattern.chars(){
//         position+=1;
//         if c == increment_floor{
//             result+=1;
//         }else if c == decrement_floor{
//             result-=1;
//         }
//         if result == -1{
//             return position;
//         }
//     }
//     position
// }
// fn match_practice(shape: &Shape){
//     match shape{
//         Shape:Square(Square) => {
//             let area = 
//         }
//     }
// }