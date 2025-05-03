/*
problem statement:

write a function that takes a string of words separated by spaces 
and return the first word it finds in string.
if no space in whole string and the return whole string.

*/

/* 
String slices
string slices is a reference to part of a string
*/

pub fn my_slice_type_function() {

    println!(".........Inside the my slice type function............");

    let s = String::from("Hello World!");

    let res = find_first_word(&s);

    //s.clear();

    println!("For string {s} the result is {res}");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Hello = {hello}");
    println!("World = {world}");

}

fn find_first_word(input: &str) -> &str {

    let s = input.as_bytes();

    for (i,&item) in s.iter().enumerate() {

        if item == b' ' {

            return &input[..i];
        }
    }

    &input[..]
}