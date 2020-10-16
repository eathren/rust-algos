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

// We have two states. One where first number is greater than second, and then one where second is greater than first.
pub enum State {
    Default,
    S0,
    S1
  }

// use match case, turn roman numerals to an int value
  pub fn char_to_int(c: char) -> i32 {
    match c {
      'I' => 1,
      'V' => 5,
      'X' => 10,
      'L' => 50,
      'C' => 100,
      'D' => 500,
      'M' => 1000,
        // _ denotes "everything else" that is not matched. 
      _ => 0,
    }
  }
  
//   impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
      // You can read more about rust options here https://doc.rust-lang.org/std/option/
      let mut last_value: Option<i32> = None;

      // no numbers are called yet, so we initialize state as Default.
      let mut state = State::Default;

      let mut sum: i32 = 0;
      for s in s.chars().rev() {
        let value = char_to_int(s);
        match state {
          State::Default => {
            state = State::S0;
            sum += value;
          },
          State::S0 => {
            if value >= last_value.unwrap() {
              sum += value;
            } else {
              sum -= value;
              state = State::S1;
            }
          },
          State::S1 => {
            if value >= last_value.unwrap() {
              sum += value;
              state = State::S0;
            } else {
              panic!("Invalid input.");
            }
          }
        }
        last_value.replace(value);
      }
      sum
    }


// }



     
        

fn main(){
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!( roman_to_int("XV".to_string()), 15);
      assert_eq!( roman_to_int("LXVII".to_string()), 67);
      assert_eq!( roman_to_int("IV".to_string()), 4);
      assert_eq!( roman_to_int("MCMXCIV".to_string()), 1994);
    }    
}

// https://leetcode.com/problems/roman-to-integer/

//  notes and tips
// it seems that in rust, ' ' is used for chars, and " " is for strings. THis caused a hangup in my match case setup.