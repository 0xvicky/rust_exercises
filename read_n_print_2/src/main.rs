use std::io;
fn main() {
    println!("Read_n_print_2");
    let mut input = String::new();
    
    println!("Enter the input:");
    match io::stdin().read_line(&mut input){
        Ok(n)=>{
            println!("Data : {n}")
    }
        Err(err) =>{
            println!("Error occured : {err}");
        }
    }   

    println!("Input : {input}");

}
