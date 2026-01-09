use std::io;

fn main() {
    println!("Even Odd 5");
    println!("Enter the input:");
    //take input from user and parse it to i32
    let mut num_raw = String::new();
    io::stdin().read_line(&mut num_raw).expect("Error input");
    let num =match num_raw.trim().parse::<i32>(){
        Ok(n) => n,
        Err(e)=>{
            eprintln!("{e}");
            return;
        }
    };

    //if n mod 2 = 0 even, else odd.
    if num % 2 == 0 {println!("Even input")} else{ println!("Odd Input")};
}
