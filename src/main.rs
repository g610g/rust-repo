use os_school::aoc;
fn main() {
    let res: &str;
    let main_string = "mainstring";
    res = bar(main_string);
    println!("{res}");
    aoc::say_hello();
    aoc::day_one::testing();
    // {
    //     let my_ref = "Hello world";
    //     let my_second_ref = "Hello another world";
    //     res = foo(my_ref, my_second_ref);
    // }
}
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    return y;
}
fn bar<'a>(foreign: &'a str) -> &'a str {
    let local_string = "local string";
    return foo(foreign, local_string);
}
