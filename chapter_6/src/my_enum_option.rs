pub fn my_option() {
    let op = Some(1);
    let a = None;
    let x = 2;
    let sum = x + op.unwrap_or(0);
    let sum1 = x + a.unwrap_or(0);

    println!("Sum is :{sum}");
    println!("Sum is :{sum1}");
    
}