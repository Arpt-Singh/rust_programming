struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8,u8,u8);
struct Point(u8,u8,u8);

pub fn my_struct_function() {
    let mut user_1 = User {
        email: String::from("apt@gmail.com"),
        username: String::from("Arpit"),
        active: true,
        sign_in_count: 0,
    };
    println!("The name of the user is {}",user_1.username);
    println!("The active state of user one is : {}",user_1.active);
    println!("The sign in count for user one is : {}",user_1.sign_in_count);
    println!("The email id of user one is: {}",user_1.email);

    println!(".............................................................................");

    /*mutate the struct*/
    user_1.username = String::from("Apt Singh");
    println!("changed user name is {}",user_1.username);

    println!(".............................................................................");

    /*another method to create an instance of a struct*/
    
    let user_2 = build_user(
        String::from("aptsngtwo"),String::from("apttwo@email.com")
    );

    println!("The name of the user two is {}",user_2.username);
    println!("The active state of user two is : {}",user_2.active);
    println!("The sign in count for user two is : {}",user_2.sign_in_count);
    println!("The email id of user two is: {}",user_2.email);

    println!(".............................................................................");

    /*creting instance from other instances*/

    let mut user_3 = User {
        email: String::from("new user from user one"),
        ..user_1
    };

    println!("The name of the user which creted from user one is {}",user_3.username);
    println!("The active state of user which creted from user one is : {}",user_3.active);
    println!("The sign in count for user which creted from user one is : {}",user_3.sign_in_count);
    println!("The email id of user which creted from user one is: {}",user_3.email);

    println!(".............................................................................");


    /*example set bg color*/

    let red = Color(100,0,0);

    set_bg_color(red);

    println!(".............................................................................");

    let point = Point(10,20,30);
    move_point(point);

}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0,
    }
}

fn set_bg_color(color: Color) {

    println!("Setting background color R={}, G={}, B={}",color.0,color.1,color.2);
}

fn move_point(point:Point) {
    println!("The cursor was moved Y= {},X={},Z={}",point.0,point.1,point.2);
}