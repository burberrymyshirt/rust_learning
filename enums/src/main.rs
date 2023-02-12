fn main() {
    println!("Hello, world!");
    
let loopback = IpAddr::V6(String::from("::1"));
let home = IpAddr::V4(127, 0, 0, 1);

fn route (ip_kind: IpAddr) {}
}

enum IpAddr {
    V4(u8, u8, u8, u8), 
    V6(String), 
}


