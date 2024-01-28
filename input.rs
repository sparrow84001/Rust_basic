use std::io;
fn main() {
    println!("hello world!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("faild to read");
    println!("{}", input);
}