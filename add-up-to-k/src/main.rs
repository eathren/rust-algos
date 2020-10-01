fn main() {
    let array:[i32; 4]= [10, 15, 3, 7];
    let k = 17;
    let mut solved = false;
   
    // for pair in array.chunks(2) {
    //     println!("({}, {})", pair[0], pair[2]);
    // }

    for number in array.iter(){
       if number % 2 == 0{
           println!("Even")
       }
       else {
           println!("Odd")
       }
    }

    // for (i,x) in array.iter(){
    //     println!("Item {} = {}", i, x);
    // }



}


  