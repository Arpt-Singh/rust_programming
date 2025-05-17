pub fn my_lifetime_function() {
    let x = 5;
    let result:&i32;

    {
        let y = 10;
        result = &y;
        //println!("result: {result}");
    }
    //error: dangling reference-> println!("result: {result}");

    let s1 = String::from("Arpit Singh");
    let s2 = String::from("Raja Singh");

    let result = longest(&s1,&s2);
    println!("longest string is {result}");


}

fn longest<'a>(x: &'a str, y:&'a str) ->&'a str {
    if x.len() > y.len() {
        return x;
        
    }
    y
}