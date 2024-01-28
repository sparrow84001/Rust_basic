use std::io;
fn main() {
    //let x:i8=9; //0 - 255
    //let y:i8=10; // -128 -12

    //let z=x+y;
    //println!("{}",z);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");

    let int_input:i64=input.trim().parse().unwrap();

    println!("{}",int_input +2);


}