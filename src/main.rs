use std::io;
use std::collections::HashMap;

fn main() {

    let mut v:Vec<isize> = Vec::new();
    let mut map = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == String::from("exit") {
            let a = Calc_Avg( & mut v);
            let most = most_Open(& map);
            println!("resut : {:?}, {:?}", a, most);

            break;
        }
        else{
            let input = input.trim().parse().expect("please insert number!");
            v.push(input);
            let count = map.entry(input).or_insert(0);
            *count += 1;
        }
    }
}

fn most_Open (map: & HashMap<isize, isize>) -> Option<& isize> {
    map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

fn Calc_Avg(arr: &  mut Vec<isize>) -> (f32,isize) {
    println!("array : {:?}", arr);
    if arr.len() == 0 {
        return (0.0,0);
    }
    arr.sort();
    println!("sorting result {:?}", arr);
    let mid_index = arr.len() / 2;
    let sum:isize = arr.iter().sum();
    
    let avg:f32 = (sum as usize / arr.len()) as f32;
    (avg,arr[mid_index])
}
