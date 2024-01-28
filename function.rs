fn main(){
    println!("hello world");
    test1();
    add_numbers(20,30);
    let result = return_addnumbers(2,3);
    println!("result {}", result);

}

fn test1(){
    println!("test has been called");
}

fn add_numbers(x:i32,y:i32){
    println!("the sum is {}",x+y);
}
fn return_addnumbers(x:i32,y:i32) ->i32{
    x+y

}