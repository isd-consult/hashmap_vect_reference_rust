    use std::collections::HashMap;
    pub mod string_ref;
    pub fn most_open (map: & HashMap<isize, isize>) -> Option<& isize> {
        map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
    }
    
    pub fn calc_avg(arr: &  mut Vec<isize>) -> (f32,isize) {
        println!("array : {:?}", arr);
        if arr.len() == 0 {
            return (0.0,0);
        }
        arr.sort();
        println!("sorting result {:?}", arr);
        let mid_index = arr.len() / 2;
        let sum:isize = arr.iter().sum();
        
        let avg:f32 = sum as f32 / arr.len() as f32;
        (avg,arr[mid_index])
    }

