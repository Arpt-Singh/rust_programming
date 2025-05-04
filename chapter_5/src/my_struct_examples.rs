#[derive(Debug)]
struct Rectangle {
    height:u32,
    width:u32,
}


pub fn example_fun() {
    
    let rect = Rectangle {
        width: 32,
        height: 50,
    };

    let area = calculate_area(&rect);

    println!("The area of the rect {:?} is : {}",rect,area);


}

fn calculate_area(rect: &Rectangle) ->u32 {

    rect.width * rect.height
}