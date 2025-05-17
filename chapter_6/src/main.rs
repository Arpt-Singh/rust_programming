pub mod my_enum;
pub mod my_enum_struct;
pub mod my_enum_with_str_data;
pub mod my_enum_option;
pub mod my_match_control_flow;
pub mod my_if_let;

fn main() {
    println!("Hello, world! In chapter 6 we will uncover the enum, Some & match type data structure in rust");

    println!(".............................................................................");

    my_enum::my_enum_function();

    println!(".............................................................................");

    my_enum_struct::my_enum_struct_function();

    println!(".............................................................................");
    
    my_enum_with_str_data::my_enum_with_str_data_fnction();

    println!(".............................................................................");

    my_enum_option::my_option();

    println!(".............................................................................");

    //my_match_control_flow::my_match_control_flow_function();
    my_if_let::my_if_let_function();


}
