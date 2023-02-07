fn main() {
    println!("Hello, world!");
}

enum IpAddr {
    V4(u8, u8, u8, u8), 
    V6(String), 
}

let loopback = IpAddr::V6(String::from("::1"));
let home IpAddr::V4(String::from(127, 0, 0, 1));

fn route (ip_kind: IpAddressKind) {}

route(four);

