use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing Game!!");
    let actual_number:u32 = 32;
    loop{
        println!("Hey User !! ENTER YOU UESS -->");
        let  mut guess : String =  String::new();
        io::stdin().read_line(&mut guess).expect("NO user input!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>{
                println!("Please enter valid input!!");
                continue;
            }
        };
        println!("Your guess {guess}");
        match guess.cmp(&actual_number) {
            Ordering::Equal => {println!("You Gussed it Right\nYOU WON !! :)");
                                    break;},
            Ordering::Less => println!("Too small :("),
            Ordering::Greater => println!("Too big :(")
        }
}

}
