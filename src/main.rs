// use std::io;
#[derive(Debug)]
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
struct Grades(u32, u32);
fn main() {
    let mut user = User{
        username:String::from("g6i1o0"),
        active:true,
        email:String::from("gio.gonzales@carsu.edu.ph"),
        sign_in_count:1
    };
    let gio_grade = Grades(98, 97);
    println!("{}", gio_grade.0);
    let Grades(first_grade, second_grade) = gio_grade;
    user.email = String::from("gio@gmail.com");
    // steal_struct(user);
    // println!("{:?}", user);
    let user2 = build_user(dbg!(user));
    println!("{}", user2.sign_in_count);
    println!("{}", user2.email);   
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
