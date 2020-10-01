use std::collections::HashMap;


// This problem was recently asked by Google.

// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

// Bonus: Can you do this in one pass?

fn main() {
    // see note 1
    let numbers= vec! [10, 15, 3, 7];
    let k = 17;
   
   // Approach 1: Brute force via iterating through each element, then loop through each element again to 
   // see if sums add up to k

    for i in 0..numbers.len(){
        for j in (i+1)..numbers.len(){
            if numbers[j] == k - numbers[i]{
                return println!("{} & {} add up to {}", numbers[i], numbers[j],k);
            }
        }
    }

}

// notes

// 1
// Vector is a 'growable' array. I used it here for easy iteration and accessibility in the iteration
// docs: https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/arrays-vectors-and-slices.html


  