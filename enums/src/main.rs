
enum IpAddrKind{
    V4(String),
    V6(u32),
}

fn main() {
    println!("Hello, world!");

    let f = IpAddrKind::V4;
    let s = IpAddrKind::V6;

    let a = Some(5i8);
    let b = Some("abc");
    let null : Option<u32> = None;
    let x : i8 = 3;

    let sum = x + a;
}
