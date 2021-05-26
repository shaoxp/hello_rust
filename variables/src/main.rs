use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);

    x = 6;
    println!("The value of x is {}",x);

    const MAX_POINTS : u32 = 1000_000;
    println!("The value const is {}",MAX_POINTS);

    let  spaces = " ";
    let spaces = spaces.len();
    println!("space is {}",spaces);

    let tup : (i32,f64,u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("the value of y is: {}",y);

    println!("value of tupe is ({},{},{})",tup.0,tup.1,tup.2);

    let a : [i32;100] = [1;100];


    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read line");
    let i :usize = i.trim().parse().expect("You enter is not a number");

    println!("panic:{}",a[i]);
}
