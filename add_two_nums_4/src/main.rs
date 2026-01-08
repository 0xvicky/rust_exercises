use std::io;

fn parse_str_to_int(inp_buf: &str) -> (Vec<i32>, Vec<String>) {
    let mut valids: Vec<i32> = Vec::new();
    let mut invalids: Vec<String> = Vec::new();

    let inp_itr = inp_buf.split_whitespace();

    inp_itr.for_each(|f| match f.parse::<i32>() {
        Ok(d) => {
            valids.push(d);
        }
        Err(_) => {
            invalids.push(f.to_string());
        }
    });
    (valids, invalids)
}
fn main() {
    println!("Add nums from input_4");
    //prompt user to which operation to perform
    println!("Enter the operation you want to perform:\n1. Addition\n2. Multiplication\n");
    let mut oprn_id_raw = String::new();
    io::stdin()
        .read_line(&mut oprn_id_raw)
        .expect("Error while taking input.");
    let oprn_id = oprn_id_raw.trim().parse::<i32>().expect("Failed to parse oprn");
    if ![1,2].contains(&oprn_id) {
      println!("Invalid operation !");
      return
    }
    //prompt user to enter numbers separated by space
    println!("Enter the numbers separated by space:😏");
    let mut inp_buffer = String::new();
    //take input buffer from io and convert it into an iterator
    io::stdin()
        .read_line(&mut inp_buffer)
        .expect("Error occured while taking input");
    //map over the iterator and first parse and then add it to result
    // let inp_itr:i32 = inp_buffer.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).sum();
    let (valids, invalids) = parse_str_to_int(&inp_buffer);

    let mut res: i32 = 0;

    //perform operation on valid vec based on operation id 
    match oprn_id{
      1 =>{ 
        res = valids.into_iter().sum();
      }
      2=>{
        res = valids.into_iter().product();
      }
      
    _ => println!("Invalid Operation")

    }
    //print the result
    println!("Result : {res}");
    println!("================================================================");
    println!("Stats");
    println!("================================================================");
    //print all the invalid inputs
    println!("Invalid Inputs:{}", invalids.len());
    invalids.iter().for_each(|x| println!("{}", x.trim()));
}

/*
• Reject invalid input and print which value failed ✅
*/
