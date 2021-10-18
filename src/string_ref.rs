// pub mod average;

// pub mod string_ref{
    pub fn convert_pig_latin(param: & mut String) ->String {
        let first_letter = param.chars().next();
        match first_letter {
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') | Some('y') => {
                // let origin = param;
                param.push_str("-hay");
                return param.clone();
            } _ => {
                let first_letter = param.remove(0);
                param.push_str("-");
                param.push(first_letter);
                param.push_str("ay");
                param.clone()
            }
        }
    }
// }