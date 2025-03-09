fn main() {
    let x: i32 = 46; //let is a keyword that helps binds a value with a vairable and this are default are immutable
    println!("value {x}");
    let mut y:i32 = 64; //mut keyword is used to make the variables mutable
    println!("y vale {y}"); //OP = 64
    y = 73;
    println!("new value of y is {y}"); // OP = 73

    // constants
    // mut is not allowed to use with constants, and instead of let , const keyword is used
    // and the type should be given
    const PI:f64= 3.14;
    println!("vale of pi {PI}");


    //shadowing 
    // can create a new var with the same name as existing var
    // diff from mutability and also can change its type
    let hrs = 60;
    println!("hrs {hrs}");   //value before shadowing
    let hrs = hrs * 2;
    println!("hrs {hrs}");    // value after shadowing
}
