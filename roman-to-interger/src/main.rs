// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.

 

// Example 1:

// Input: s = "III"
// Output: 3
// Example 2:

// Input: s = "IV"
// Output: 4
// Example 3:

// Input: s = "IX"
// Output: 9
// Example 4:

// Input: s = "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// Example 5:

// Input: s = "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 

// Constraints:

// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].



// BREAKDOWN
// If number to the right is =< the number on the left, add values together.
// if number on the left is > the number on the right, subtract the value on the left from the value on the right
// it appears that if an odd number, the numbers are counted in pairs, excluding the first number.

// step 1, iterate over the string, splitting it.
// step 2, match the values of each split 'string' in the previous string, and match each numeral to its numeric value.
// step 3, iterate over the new int vec, and compare the values. Apply first logic of positive or negative values, and sum it
// step 4, return the sum

fn roman_to_int(s: String)  {
  
    

    // let split_string : Vec<&str> =  s.chars().collect();
    let mut number = 0;
    let v: Vec<&str> = s.split("").collect();
    let mut num_vec: Vec<i32> = Vec::new();
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
    println!("{}", num_vec[1]);

    let mut final_number:i32 = 0;

    for i in 0..num_vec.len(){
        
        let j = i + 1;
        if final_number == 0 && num_vec[i] >= num_vec[j]  {
            final_number +=    num_vec[i] + num_vec[j];
            println!("0. Greater than. Final number is {}", final_number);
            continue;
        }
        else if final_number >  0 && num_vec[i] >= num_vec[j]{
            println!("{}, {}", num_vec[i], num_vec[j]);
            final_number +=    num_vec[j];
            println!(">0. Greater than. Final number is {}", final_number);
            continue;
        }
        else if final_number >  0 && num_vec[i] < num_vec[j]{
            println!("{}, {}", num_vec[i], num_vec[j]);
            final_number -=    num_vec[j];
            println!("{}", final_number);
            continue;
        }
        else if j >= num_vec.len(){
             final_number;
        }
            
    }
    println!("{}", final_number)
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
