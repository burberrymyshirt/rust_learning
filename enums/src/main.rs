fn main() {
    println!("Hello, world!");
    
let _loopback = IpAddrKind::V6(String::from("::1"));
let _home = IpAddrKind::V4(127, 0, 0, 1);
}

enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String), 
}

fn _route (_ip_kind: IpAddrKind) {}
