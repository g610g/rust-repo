//use std::collections::HashMap;
//
//fn create_phone_number(numbers: &[u8]) -> String {
//    let mut result_string: String = String::new();
//    numbers.iter().enumerate().for_each(|(idx, val)| {
//        let casted = char::from_digit(*val as u32, 10).expect("Errr converting u8 to u32");
//        match idx {
//            0 => {
//                result_string.push('(');
//                result_string.push(casted);
//            }
//            2 => {
//                result_string.push(casted);
//                result_string.push(')');
//                result_string.push(' ');
//            }
//            5 => {
//                result_string.push(casted);
//                result_string.push('-');
//            }
//            _ => result_string.push(casted),
//        }
//    });
//
//    result_string
//}
//fn morse_code_decoder(morse_code: &str, map: HashMap<String, String>) -> String {
//    let mut space_count = 0;
//    let mut buffer = String::new();
//    morse_code.chars().fold("".to_string(), |acc, curr| {
//        if space_count == 3 {
//            return acc + " ";
//        } else if space_count == 1 && !curr.is_whitespace() {
//            let res = map.get(&buffer).expect("No entry");
//            buffer.clear();
//            return acc + res;
//        }
//        space_count += 1;
//        buffer.push(curr);
//        acc
//    })
//}
//fn main() {
//    let numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
//    let map: HashMap<String, String> = HashMap::new();
//    // morse_code_decoder(".... . -.--   .--- ..- -.. .", map);
//    // let str_result = create_phone_number(&numbers);
//    // println!("Result:{str_result}");
//    let string_res = reverse_string("The Quick Brown Fox");
//    println!("{}", string_res);
//    let my_vec = vec![1, 2, 3, 5];
//}
//fn camel_case(text: &str) -> String {
//    let res = text
//        .split(&['-', '_'][..])
//        .enumerate()
//        .map(|(idx, item)| {
//            if idx == 0 {
//                return item.to_string();
//            }
//            let mut c = item.chars();
//            match c.next() {
//                None => String::new(),
//                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//            }
//        })
//        .collect::<String>();
//    return res;
//}
//fn reverse_string(str: &str) -> String {
//    str.split(" ")
//        .map(|item| item.chars().rev().collect::<String>())
//        .collect::<Vec<String>>()[..]
//        .join(" ")
//}
//fn decode_morse(encoded: &str) -> String {
//    let mut space_count = 0;
//    let mut buffer = String::new();
//    encoded.chars().fold("".to_string(), |acc, curr| {
//        if space_count == 3 && !curr.is_whitespace() {
//            let res = MORSE_CODE.get(&buffer).expect("No entry");
//            space_count = 0;
//            buffer.clear();
//            buffer.push(curr);
//            return acc + res + " ";
//        } else if space_count == 1 && !curr.is_whitespace() {
//            let res = MORSE_CODE.get(&buffer).expect("No entry");
//            buffer.clear();
//            space_count = 0;
//            buffer.push(curr);
//            return acc + res;
//        } else if curr.is_whitespace() {
//            space_count += 1;
//            return acc;
//        }
//        buffer.push(curr);
//        acc
//    })
//}
