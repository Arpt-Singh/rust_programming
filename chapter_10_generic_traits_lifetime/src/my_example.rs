
#[drive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}


impl<T,U> Point<T,U> {
    fn new(x:T,y:U) -> self {
        self {x, y}
    }

    fn mixup<X,Y>(self, point: Point<X,Y>) -> Point<T,Y> {
        Point{
            x: self.x,
            y:point.y,
        }
    }
}
fn largest<T: std::cmp::PartialOrd>(list:&[T]) ->&T{
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

pub fn my_example_function() {
    let list = vec![1,5,7,10,20,60];
    let l = largest(&list);

    let list_two = vec![2.0,2.2,10.5,20.7,90.3,100.09];
    let l_two = largest(&list_two);

    println!("largest number is : {l}");
    println!("largest number is : {l_two}");

    let point_a = Point {x: 10, y:20};
    let point_b = Point{x:10.5,y:20.7};
    /*
    let point_a = Point::new(6.3,5);
    let point_b = Point::new(10.5,20};
    */
    let point_c = Point::new(4.6,5);

    let point_d = point_b.mixup(point_b);

    println!("Point D is :{point_d:?}");



}