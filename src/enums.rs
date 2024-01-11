
pub struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
#[derive(Debug)]
pub struct Dog{
    name:String,
    age:u8,
    breed_type:String,
    color:String,
    height: f32,
    is_alive:bool
    
}
#[derive(Debug)]
pub enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub struct Square{
    pub sides:u64
}

pub struct Circle{
    pub radius: f64
}

pub struct Triangle{
    base: u64,
    height:u64
}
pub struct Rectangle{
    length:u64,
    width:u64
}
pub enum Shape{
    Square(Square),
    Triangle(Triangle),
    Circle(Circle),
    Rectangle(Rectangle),
}
pub enum Number{
    Integer(u64),
    Float(f64)
}
impl Shape{
    pub fn determine_area(&self) -> Number {
        match self{
            Shape::Square(Square) => {
                let area = Square.sides.pow(2);
                println!("The shape is a square and the area is: {}", area);                
                Number::Integer(area)
            },
            Shape::Triangle(Triangle) => {
                let area = (Triangle.height * Triangle.base) * 2;
                println!("The shape is a triangle and the area is: {}", area);
                Number::Integer(area)
            },
            Shape::Circle(Circle) => {
                let pi = 3.14;
                let area = pi * Circle.radius.powf(2.0);
                println!("The shape is circle and the area is :{}", area);
                Number::Float(area)
            }
            Shape::Rectangle(Rectangle) => {
                let area = Rectangle.length * Rectangle.width;
                println!("The shape is rectangle and the area is :{}", area);
                Number::Integer(area)
            }
        }
    }
}

impl Message{
    pub fn call(&self){
        dbg!(&self);
    }
}
 impl Dog{
    pub fn bark(&self){
        println!("Woof!");
    }
    pub fn say_name(&self){
        println!("My name is {}", self.name);
    }
    pub fn create_dog(name:String, age:u8, breed_type:String, color:String, height:f32, is_alive:bool) -> Self{
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
