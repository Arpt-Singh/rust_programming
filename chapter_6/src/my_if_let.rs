pub fn my_if_let_function() {
    let config_max = Some(3_u8);
    /*match config_max {
        Some(max) => println!("The maximum is configured to be {max}");
        _ => (),
    }*/
    if let Some(num) = config_max {
        println!("The maximum is configured to be {max}");

    }

}