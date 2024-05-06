fn main() {
    let returned = baz(Some(2)).unwrap();
    println!("{}", returned);
}
// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x;
//     }
//     return y;
// }
// fn bar<'a>(foreign: &'a str) -> &'a str {
//     let local_string = "local string";
//     return foo(foreign, local_string);
// }
fn baz(opts: Option<usize>) -> Option<usize> {
    return opts.map(|x| x * 5);
}
