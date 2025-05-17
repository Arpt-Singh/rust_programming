pub fn my_str_function() {
    let mut hello = String::from("Namaste ");
    hello.push('@'); // char push
    hello.push_str("Piyush"); // string push
    println!("{hello}");
    //let first_char = hello[0]; // not possible
    //println!("{first_char}");
}