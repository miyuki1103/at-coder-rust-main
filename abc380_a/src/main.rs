use std::{collections::HashMap, io::{stdin, BufRead}};

// https://atcoder.jp/contests/abc380/tasks/abc380_a
pub fn main(){
    // number: u64
    let std= stdin();
    let mut buf=std.lock().lines();
    let string_input= buf.next().unwrap().unwrap();
    let number_input= string_input.parse::<u64>().unwrap();

    let n_string= number_input.to_string().trim().to_string();
    let v_n: Vec<char>= n_string.chars().collect();
    
    let mut map_one: HashMap<char, u64>= HashMap::new();
    let mut map_two: HashMap<char, u64>= HashMap::new();
    let mut map_three: HashMap<char, u64>= HashMap::new();

    for e_n in v_n {
        match e_n {
            '1' => {
                map_one.insert('1', 1);
            },
            '2' => {
                map_one.insert('2', 1);

                if *(map_one.get(&'2').unwrap()) == 1 {
                    map_two.insert('2', 2);
                }
            },
            '3' => {
                map_one.insert('3',1);

                if *(map_one.get(&'3').unwrap()) == 1 {
                    map_two.insert('3', 2);
                    
                }
                if *(map_two.get(&'3').unwrap()) == 2 {
                    map_three.insert('3', 3);
                }
            },
            _ => { println!("No"); }
        }
    }



    if *(map_one.get(&'1').unwrap()) == 1 
        && *(map_two.get(&'2').unwrap()) == 2 
        && *(map_three.get(&'3').unwrap()) == 3  {
            println!("Yes");
    }
}
