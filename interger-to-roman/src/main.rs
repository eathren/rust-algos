// OVERVIEW

// We are doing the opposite of https://github.com/eathren/rust-algos/blob/master/roman-to-interger/src/main.rs !



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
// Given an integer, convert it to a roman numeral.

 

// Example 1:

// Input: num = 3
// Output: "III"
// Example 2:

// Input: num = 4
// Output: "IV"
// Example 3:

// Input: num = 9
// Output: "IX"
// Example 4:

// Input: num = 58
// Output: "LVIII"
// Explanation: L = 50, V = 5, III = 3.
// Example 5:

// Input: num = 1994
// Output: "MCMXCIV"
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.


// BREAKDOWN
// step 1. Take the input of an int. Convert it to a string, and split it. 
// step 2. Iterate over the chars. 
// step 3. Return a roman numeral in the correct order according to our state and rules.

pub enum State {
    Default,
    S0,
    S1
  }
  
  pub fn string_to_char(c: &str) -> char {
    match c {
      "1" => 'I',
      "5" => 'V',
      "10" => 'X',
      "50" => 'L',
      "100" => 'C',
      "500" => 'D',
      "1000" => 'M',
      &_ => println!("false value entered, {}", c)
     
    }
  }

    pub fn int_to_roman(num: i32) -> String {
        let new_string = num.to_string();
        for s in new_string.split("").rev(){
            let value = string_to_char(new_string);
        }

        
    }

fn main() {
    println!("Hello, world!");
}
