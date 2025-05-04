#[derive(Debug)]
struct Rectangle {
    height:u32,
    width:u32,
}

impl Rectangle {
    fn calculate_area(&self) ->u32 {
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle) ->bool {
        self.width >= other.width && self.height >= other.height

    }
    fn square(size:u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn my_method_function() {
    
    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 35,
        height: 30,
    };


    println!("The area of the rect {:?} is : {}",rect1,rect1.calculate_area());

    println!("The area of the rect {:?} is : {}",rect2,rect2.calculate_area());

    println!("Can rect1 hold rect2 ? {}",rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);

    println!("sequare fron rectangle is : {:?}",sq);


}

fn calculate_area(rect: &Rectangle) ->u32 {

    rect.width * rect.height
}