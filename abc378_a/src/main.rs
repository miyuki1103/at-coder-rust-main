use std::{collections::HashMap, io::{stdin, BufRead}};

// https://atcoder.jp/contests/abc378/tasks/abc378_a
fn main(){
    // numbers: Vec<i32>
    let std= stdin();
    let mut buf= std.lock().lines();
    let string_input= buf.next().unwrap().unwrap();
    let numbers: Vec<i32>= string_input.split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let mut hashmap : HashMap<i32, i32>= HashMap::new();
    let mut hashmap_full : HashMap<i32, i32>= HashMap::new();
    let mut hashmap_three : HashMap<i32, i32>= HashMap::new();
    let mut hashmap_full2 : HashMap<i32, i32>= HashMap::new();
    let mut cnt: i32= 0;
    for e_n in numbers {
        if hashmap_full.contains_key(&e_n) && cnt == 1{
            hashmap_three.insert(e_n, 3);
        }
        
        if hashmap_three.contains_key(&e_n) {
            cnt+=1;
        }

        if hashmap.contains_key(&e_n){
            hashmap_full.insert(e_n, 2);
            cnt+=1;
        }
        hashmap.insert(e_n, 1);
    }

    println!("{}", cnt);
}
