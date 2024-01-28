fn main(){
    let mut x=4;
    println!("x is : {}",x);

    {
        let x=2;
        println!("x is : {}",x);
    }
    let x="hello";
    println!("x is : {}",x);

    //construct
    const SECOND_IN_MINUTE:u32=60;
    println!("SECOND_IN_MINUTE is : {}",SECOND_IN_MINUTE)
 }