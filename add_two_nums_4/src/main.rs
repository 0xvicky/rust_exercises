use std::io;    

fn parse_str_to_int(inp_buf: &str) -> (Vec<i32>, Vec<String>){
    let mut valids:Vec<i32> = Vec::new();
    let mut invalids:Vec<String> = Vec::new();

      let inp_itr = inp_buf.split_whitespace();

      inp_itr.for_each(|f| {
        match f.parse::<i32>()
      {  Ok(d)=>{
          valids.push(d);
      }
        Err(_)=>{
            invalids.push(f.to_string());
        }
    }
      });
    (valids, invalids)
}
fn main() {
    println!("Add nums from input_4");
    //prompt user to enter numbers separated by space
    println!("Enter the numbers separated by space:😏");
    let mut inp_buffer = String::new();
    //take input buffer from io and convert it into an iterator
    io::stdin().read_line(&mut inp_buffer).expect("Error occured while taking input");
    //map over the iterator and first parse and then add it to result
    // let inp_itr:i32 = inp_buffer.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).sum();
    let (valids, invalids) = parse_str_to_int(&inp_buffer);
    let res:i32 = valids.into_iter().sum();

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