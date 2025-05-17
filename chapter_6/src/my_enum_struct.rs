#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}
struct IpAddress {
    address: String,
    kind: IpAddrKind,
}

impl IpAddress {
    fn new(address:&str) ->Self {
        Self {
            address: address.to_string(),
            kind: IpAddrKind::v4,
        }
    }
}
pub fn my_enum_struct_function() {

    let generate_ip = IpAddress::new("1.2.3.4");

    route(generate_ip);

}
fn route(ip: IpAddress) {
    println!("Routing request Ip {} of kind {:?}",ip.address,ip.kind);
}