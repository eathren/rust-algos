// QUESTION

// Check to see if a string has the same amount of 'x's and 'o's. The method must return a boolean and be case insensitive. The string can contain any char.

// Examples input/output:

// XO("ooxx") => true
// XO("xooxx") => false
// XO("ooxXm") => true
// XO("zpzpzpp") => true // when no 'x' and 'o' is present should return true
// XO("zzoo") => false

// BREAKDOWN

// We need to take a string, count the x's and o's, and returns. 

//The first problem is to count both capital and undercase letters. We solve this by 
// making everything lowercase, but in a case-sensitive setting, we'd do an or match instead. 

// next, we count the x's in one var, and the o's in another. 
// for the last step, do a == compare to check if the string count is equal. If they are, it returns true.
// which passes the test successfully. 

fn main() {
    
}

fn xo(string: &'static str) -> bool {
    let string = string.to_lowercase();
    let x_count= string.matches('x').count();
    let o_count = string.matches('o').count();
    x_count == o_count
    
 }

 #[test]
fn returns_expected() {
  assert_eq!(xo("xo"), true);
  assert_eq!(xo("Xo"), true);
  assert_eq!(xo("xxOo"), true);
  assert_eq!(xo("xxxm"), false);
  assert_eq!(xo("Oo"), false);
  assert_eq!(xo("ooom"), false);
}