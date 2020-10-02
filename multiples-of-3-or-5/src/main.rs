// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in.

// Note: If the number is a multiple of both 3 and 5, only count it once. Also, if a number is negative, return 0(for languages that do have them)

// Courtesy of projecteuler.net


fn main() {
    // declare num as a signed int. If you're not sure what a signed int is,
    // check the book here -> https://doc.rust-lang.org/book/ch03-02-data-types.html
    let num:i32 = 25;
    let answer:i32 = solution(num);
    println!("{}", answer)
}

// to break this one down, first we iterate over the range, and filter out
// only the values that are divisible by 3 or 5, we then sum that number
fn solution(num: i32) -> i32 {
    (1..num).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}


