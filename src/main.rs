use std::io;
use std::collections::HashMap;
mod enums;
// use crate::enums::{Circle, Square, Number, Shape};
fn main() {
    
    let name = "Gio Gonzales";
    let key = "one";
    let mut map = HashMap::new();
    map.insert(key, 1);
    let mutable_reference = map.entry(key).or_insert(2);
    let mut my_list = vec![120, 50, 33, 47, 75, 88, 99, 2, 1, 34, 1, 1];
    let mut employee_department_map: HashMap<String, Vec<String>> = HashMap::new();
    loop{
        let mut choice = String::new();
        println!("Welcome to Department Employee Structure");
        println!("1.Add employee to the department");
        println!("2.Check employee of the department");
        println!("3. Exit");
        io::stdin().read_line(&mut choice).expect("There is an error");
        let choice: Result<u8, _> = choice.trim().parse();
        match choice{
            Ok(result) => {
                if result == 1{
                    let mut department = String::new();
                    let mut employee = String::new();
                    println!("Enter the deparment you want the employee to be added");
                    io::stdin().read_line(&mut department).expect("Error reading the stdin input");
                    println!("Enter the employee name you want to add to the inputted department");
                    io::stdin().read_line(&mut employee).expect("Error reading the stdin input");
                    add_employee(department, &mut employee_department_map, employee);
                }else if result == 2 {
                    read_map(&employee_department_map);
                }else{
                    break;
                }
        
            }
            Err(e) => {
                println!("Error");
            }
        }
    }
}
fn read_vector(slice: &[usize]){
    for i in slice{
        println!("{i}");
    }
}
fn find_median(my_list: &mut Vec<i32>) -> i32{
    my_list.sort();
    let length = my_list.len();
    println!("{length}");
    let mut mid : usize;
    let mut result: i32 = 0;
    if length % 2 == 0{
        mid = (length / 2) - 1;
        match my_list.get(mid){
            Some(i) => {result+=i},
            None => {}
        }
        match my_list.get(mid + 1){
            Some(i) => {result+=i},
            None => {}
        }
    }else{
        mid = length / 2;
        println!("{mid}");
        match my_list.get(mid){
            Some(i) => {result+=i},
            None => {}
        }
    }
    return result;
}
fn find_mode(my_list: &Vec<i32>)-> i32{
    let mut map = HashMap::new();
    let mut max:i32 = -1;
    let mut max_key: i32 = -1;
    for i in my_list{
        let count = map.entry(i).or_insert(0);
        *count+=1;
    }
    for key in map.keys(){
        match map.get_key_value(key){
            Some(i) => {
                if i.1 > &max{
                    max_key = **key;
                    max = *i.1;
                }
            }
            None => {}
        }
    }
    max_key
}
fn add_employee(department: String, map: &mut HashMap<String, Vec<String>>, employee: String){
    let department = department.trim().to_string();
    let employee = employee.trim().to_string();
    let list = map.entry(department).or_insert(vec![]);
    list.push(employee);
}
fn read_map(map: &HashMap<String, Vec<String>>){
    for list in map{
        println!("{:?}", list);
    }
}
