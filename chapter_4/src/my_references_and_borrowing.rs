pub fn my_ref_and_borrowing_function() {

    println!("..............inside my ref and borrowing function...............");

    let mut my_str = String::from("Hello From Dev Arpit!");

    let len_one = calculate_len_one(&my_str);

    println!("The len of the my string {my_str} is : {len_one}");

    let len_two = calculate_len_two(&mut my_str);

    println!("The len of the my string {my_str} is : {len_two}");



}

fn calculate_len_one(s: &String) ->usize {

    let result = s.len();

    result
}

fn calculate_len_two(s: &mut String) ->usize {

    s.push_str("push in bowrroing string"); 

    let result = s.len();

    result
}