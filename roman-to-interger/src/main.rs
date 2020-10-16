// BREAKDOWN
// If number to the right is =< the number on the left, add values together.
// if number on the left is > the number on the right, subtract the value on the left from the value on the right
// it appears that if an odd number, the numbers are counted in pairs, excluding the first number.

// step 1, iterate over the string, splitting it.
// step 2, match the values of each split 'string' in the previous string, and match each numeral to its numeric value.
// step 3, iterate over the new int vec, and compare the values. Apply first logic of positive or negative values, and sum it
// step 4, return the sum

fn roman_to_int(s: String)  {
    // for c  in s.chars(){
        
    // }

    

    // let split_string : Vec<&str> =  s.chars().collect();
    let mut number = 0;
    let v: Vec<&str> = s.split("").collect();
    let mut num_vec = Vec::new();
    for s in v {
        match s  {
            "I" =>  num_vec.push(1),
            "V"=>  num_vec.push(5),
            "X" =>  num_vec.push(10),
            "L" =>  num_vec.push(50),
            "C" =>  num_vec.push(100),
            "D" =>  num_vec.push(500),
            "M" =>  num_vec.push(1000),
            _ => println!("{} is not equal to any of these values", s)
            
        }
        println!("{}", s);
    }

    println!("{:?}", num_vec);
    // let vec = split.collect::<Vec<&str>>();
}

fn main(){
    roman_to_int("XVII".to_string());
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!( roman_to_int("xv".to_string()), 15);
    }    
}
