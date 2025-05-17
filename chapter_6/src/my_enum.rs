#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

pub fn my_enum_function() {

    let four = IpAddrKind::v4;

    let six = IpAddrKind::v6;

    route("196.168.1.1",four);

    route("196.168.1.2",six);

}

fn route(ip: &str, kind: IpAddrKind) {
    println!("Routing request to IP {ip} of kind {kind:?}");
}