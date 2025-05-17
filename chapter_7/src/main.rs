// src/main.r <-- Root Binary Crate
// src/lib.rs <-- Root Lib Crate


use chapter_7::auth_utils::models::Credentials;

use chapter_7::authentication;

fn main() {
    
    println!("..................Hello, world! In this chaptter we will see packages and crates..................");

    let cred = Credentials {
        username: String::from("arpitsingh"),
        password: String::from("1234567"),
    };

    authentication(cred);
}
