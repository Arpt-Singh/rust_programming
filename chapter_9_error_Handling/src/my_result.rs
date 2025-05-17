pub fn my_result_function() {
    let r = match divide(4,2) {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {err}");
            -1
        }
    };
    println!("{r}");
}

fn divide(x:i32,y:i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Please do not dive by zero."));
    }
    Ok(x/y)
}