use std::io;
use std::collections::HashMap;
use reference::{average, string_ref};

fn main() {
    let mut v:Vec<isize> = Vec::new();
    let mut map = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == String::from("exit") {
            let a = average::calc_avg( & mut v);
            let most = average::most_open(& map);
            println!("resut : {:?}, {:?}", a, most);

            break;
        } else if input.trim() == String::from("stringref") {
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let pigletters = string_ref::convert_pig_latin(& mut input.trim().to_string());
                    println!("{}", pigletters);
            }
        } else{
            let input = input.trim().parse().expect("please insert number!");
            v.push(input);
            let count = map.entry(input).or_insert(0);
            *count += 1;
        }
    }
}