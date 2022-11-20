use std::io;


fn add(a:i32, b:i32){
    let sum = a + b;

    println!("Sum if a and b ={}", sum);
}


fn main() {
    let mut input1 = String::new()
    println!("Enter input for parameter A :");
    io::stdin().read_line(&mut input1).expect("Fsiled to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter inoput for parameter B");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
     leg b:i32 = inout2.trim().parse().expect("Invalid input");

     // call function with arguments
     add(a ,b);
}

