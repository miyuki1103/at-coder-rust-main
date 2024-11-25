use std::io::{stdin, BufRead};

fn main(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let str_input= buf.next().unwrap().unwrap();
    let number: u64= str_input.parse().unwrap();
    let chars: Vec<char>= number.to_string()
        .chars()
        .collect();

    let string_one= format!("{}{}{}", chars[1], chars[2], chars[0]);
    let string_two= format!("{}{}{}", chars[2], chars[0], chars[1]);

    let number_1: u64= string_one.parse().unwrap();
    let number_2: u64= string_two.parse().unwrap();

    println!("{} {}", number_1, number_2);
}⏎  