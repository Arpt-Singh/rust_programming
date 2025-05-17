#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn my_vec_function() {
    let mut vec_num = Vec::new();
    let mut vec_chr = Vec::new();
    let mut vec_init = vec![7,8,9,0];
    let mut vec_change = vec![1,7,9,8];

    let my_vec:Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Float(223.1),
        SpreadSheetCell::Text(String::from("good")),
        SpreadSheetCell::Int(25),
        ];



    vec_num.push(1);
    vec_num.push(2);
    vec_num.push(3);
    vec_num.push(4);
    vec_num.push(5);

    vec_chr.push('a');
    vec_chr.push('b');
    vec_chr.push('c');
    vec_chr.push('d');
    vec_chr.push('e');

    vec_init.push(11);
    vec_init.push(12);

    let fourth_value = vec_num[3];

    let third_value = vec_num.get(2).unwrap();

    let second_value = vec_num.get(20).unwrap_or(&-1);

    let first_value = match vec_num.get(30) {
        Some(value) => value,
        None => {
            println!("The index out of bound!");
            &-1
        }
    };



    println!("vec = {:?}",vec_chr);
    println!("vec = {:?}",vec_init);

    println!("vec = {:?} and the fourth value is: {fourth_value}",vec_num);
    println!("vec = {:?} and the third value is: {third_value}",vec_num);
    println!("vec = {:?} and the first value is: {first_value}",vec_num);


    for i in &mut vec_change {
        println!("i is {i}");
        *i = *i*2;
    }
    println!("vec = {:?}",vec_change);

    println!("vec = {:?}",my_vec);

    match my_vec.get(2) {
        Some(SpreadSheetCell::Int(value)) => println!("The value is int {value}"),
        Some(_) => println!("This is some other type value"),
        None => println!("None")
    }


}