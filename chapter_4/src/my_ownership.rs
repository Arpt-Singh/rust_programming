/* 
-> Each value in rust has an owner
-> there can be one owner at a time
-> When the owner goes out of scope the value will be dropped
*/

//................................................................//

/*
-> Rust takes a differentt path: 
    memory is automatically returned once,
    the variable that owns it goes out of scope.
*/

pub fn my_ownership_function() {

    println!("...............inside my ownership fn..............");
    
    /*Stack*/

    let x = 10;

    let y = x;

    println!("No error only Stack operation:-> copying x value to y: now x is: {x} and y is: {y}");

    /* String Type (Heap matters)*/

    let mut s = String::from("Hello ");

    s.push_str("World!");

    println!("my String is: {s}");

    /*Double free*/

    let s1 = String::from("I am S1");

    //let s2 = s1; 

    //println!("S1 is {s1}"); // error after free s1 you want to try 

    let s2 = s1.clone(); // expensive (performance degrading)

    println!("S1 is {s1}"); // no error

    println!("S2 is {s2}"); // error after free s1 you want to try 

    /*example*/

    let num = 15;

    let result = add(num);

    let name = String::from(" Arpit Singh"); // heap allocated

    take_ownership(name);

    println!("Num is {num} and result = {result}");

    //println!("Value of the name is {name}"); // error 

    let s = gives_ownership();

    println!("Gives ownership gave string s: {s}");

    let s2 = takes_and_gives_back(s);

    println!("Takes and gives ownership string s: {s2} ");

    let my_str = String::from("Hello From Dev Arpit!");

    let (my_str,l) = calculate_len(my_str);

    println!("The len of the my string {my_str} is : {l}");
}

fn take_ownership(s:String) {

    println!("Inside ownership {s}");

    // s goes outof the scope and mem::drop is called. memory is freed. Now you can't access the s anywhere 
}

fn add(x:i32) ->i32 {

    x + 10

}

fn gives_ownership() ->String {

    let s = String::from("This is a string from gives ownership!");

    s

}

fn takes_and_gives_back(s: String) ->String {

    println!("S in takes and gives back ownership : {s}");

    s

}

fn calculate_len(s:String) ->(String,usize) {

    let result = s.len();

    (s, result)
}