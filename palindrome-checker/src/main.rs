// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

// Follow up: Could you solve it without converting the integer to a string?

 

// Example 1:

// Input: x = 121
// Output: true
// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
// Example 4:

// Input: x = -101
// Output: false
 

// Constraints:

// -231 <= x <= 231 - 1


fn main(){

}

    pub fn is_palindrome(x: i32) -> bool {
        if (x < 0) {
            return false;
        }
        let mut y = 0;
        let mut z = x;
        while z > 0 {
            
            y *= 10;
            y += z % 10;
            z /= 10;
        }
        y == x
    }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(1331), true);
        assert_eq!(is_palindrome(1259), false);
        assert_eq!(is_palindrome(202), true);
        assert_eq!(is_palindrome(5884), false);
        assert_eq!(is_palindrome(80008), true);
      }  
}