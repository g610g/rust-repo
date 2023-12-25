// use std::io;

fn main() {
    let s1 = String::from("This is a word");
    println!("{}", s1.len());
    // let word = first_word(&s1, s1.len());
    let expression_if = if true {"True"} else {"False"};
    // looper();
    println!("{}", fib_generator_recur(1));

}

fn fib_generator_recur(n: u32) -> u32{
    if (n == 1 || n == 0)
    {
        return n;
    }
    return fib_generator_recur(n - 1) + fib_generator_recur(n - 2);
}
fn fib_generator(n: u32)-> u32{
    let mut result = 0;
    let mut minus_1 = 1;
    let mut minus_2 = 0;
    if n == 0 {
        return 0;
    }else if n == 1 {
        return 1;
    }
    for numbers  in (1..n){
        result = minus_1 + minus_2;
        minus_2 = minus_1;
        minus_1 = result;
    }
    result

}


fn foor_loop()-> (){
    for number in (1..5){
        println!("{number}");
    }
}
fn looper()-> () {
    let mut counter = 1;
    let multiplier = 1;
    'outerloop: loop{
        println!("counter value: {}", counter);
        loop{
            println!("inner result = {}", counter + multiplier);

            if counter == 5{
                break 'outerloop;
            }
            counter += 1;
        }
        // counter += 1;

    }
}
fn first_word(s: &String, str_len: usize)-> String {
    let mut result = String::new();
    for character in s.chars() {
        if character == ' ' {
           break;                
        }
        result.push(character);
    }
    return result;        
}
