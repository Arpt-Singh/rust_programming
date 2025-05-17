#[derive(Debug)]
enum IpAddressKind {

    V4(u8,u8,u8,u8),
    
    V6(String),
}

pub fn my_enum_with_str_data_fnction() {

    let home = IpAddressKind::V4(127,0,0,1);

    route(home);

    let loopback = IpAddressKind::V6(String::from("127.0.0.1"));

    route(loopback);

}

fn route(ip:IpAddressKind) {
    println!("Routing request to {:?}",ip);
}