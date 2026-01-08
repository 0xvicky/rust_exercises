use std:: io;

use chrono::prelude::*;
fn main() {
    println!("Parse_str_int_3");
    let mut year_str = String::new();
    //prompt user enter year of birth
    println!("What's your year of birth??🙂");
    //take year as input from user
   match io::stdin().read_line(&mut year_str){
    Ok(_)=>{}
    Err(e) =>{
        println!("Error occured while taking input :{e}");
    }
   }
    //parse the string into int 
    let year_int = match year_str.trim().parse::<i32>(){
        Ok(d) => d,
        Err(e) => {
            println!("Error occured :{e}");
            return;
        }
    };

    //calculate the age of user by subtracting it with current year
    // println!("{}", {chrono::Utc::now().year()});
    let curr_year = (chrono::Utc::now().year()) - year_int;
    //print the age to the user
    println!("Your Age : {curr_year}");
}
