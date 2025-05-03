pub mod my_ownership;
pub mod my_references_and_borrowing;
pub mod my_slice_type;

fn main() {

    println!("................Inside the main function!.............");

    println!("Hello, World! We will cover ownership ref borrowing, slice and  ref slice concepts in rust!");
    
    my_ownership::my_ownership_function(); // chapter 4.1

    my_references_and_borrowing::my_ref_and_borrowing_function();  // chapter 4.2

    my_slice_type::my_slice_type_function();  // chapter 4.3

}
