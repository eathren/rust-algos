// QUESTION

// Digital root is the recursive sum of all the digits in a number.

// Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.

// Examples
//     16  -->  1 + 6 = 7
//    942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 

// note: Run with cargo test. fn main() is here simply because rust will throw 
// an error if it isn't. 

//That being said...
// BREAKDOWN

// this is in essence, a question about type conversions and iteration.
// there are three four main parts.
// 1: Check if n is single digit or not. If it is, return n, as it is the 'last' digit.
 
// next, if n >=10, convert it to a string. Then iterate over the string  using either .split("") or over the chars with .chars()
// then, get the sum as an i64.

// REMEMBER: When you unwrap, add the type to what is being unwrapped to!

fn main(){

}

fn digital_root(n: i64) -> i64  {
    // if n is 1 digit long, return n
    if n < 10 { 
        n
      }
      // convert n to string, split into chars, iterate over the string, 
      // Unwrap, map, turn the string chars into ints, and sum them. Return.
      else {
          digital_root(n.to_string().chars().into_iter().map(|c| c.to_digit(10).unwrap() as i64).sum())
      }
  }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!( digital_root(16), 7);
    }    
}
