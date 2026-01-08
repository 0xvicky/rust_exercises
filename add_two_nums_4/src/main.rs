use std::io;
fn main() {
    println!("Add nums from input_4");
    //prompt user to enter numbers separated by space
    println!("Enter the numbers separated by space:😏");
    let mut inp_buffer = String::new();
    let mut invalid_inputs:Vec<String> = Vec::new();
    //take input buffer from io and convert it into an iterator
    io::stdin().read_line(&mut inp_buffer).expect("Error occured while taking input");
    //map over the iterator and first parse and then add it to result
    // let inp_itr:i32 = inp_buffer.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).sum();

    let inp_itr:i32 = inp_buffer.split_whitespace().filter_map(|x| {
        match x.parse::<i32>(){
            Ok(num)=>{Some(num)},
            Err(_) =>{
                // println!("Error occured while parsing: {e}");
                // println!("{x}");
                invalid_inputs.push(x.to_string());
                None
            }
        }
    } 
).sum();

    //print the result
    println!("Result : {inp_itr}");
println!("================================================================");
println!("Stats");
println!("================================================================");
    //print all the invalid inputs
    println!("Invalid Inputs:{}", invalid_inputs.len());
    invalid_inputs.iter().for_each(|x| println!("{}", x.trim()));

}

/*
• Reject invalid input and print which value failed
• Accept floats instead of ints
• Reduce using .fold() instead of sum()
• Sort and print the numbers before summing
• Compute mean/median
*/