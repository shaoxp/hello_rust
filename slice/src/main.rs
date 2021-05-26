fn main() {
    println!("Hello, world!");

    println!("{}",first_world("abc def"));

    println!("{}",first_world(&String::from("zxy 123")[..]));

    println!("{}",first_world(&String::from("zxy 123")[..]));

    let b = [1,2,4,6];
    let aa = &b[1..3];
    for i in slice(aa).iter(){
        println!("{}",i);
    }
}

fn first_world(str : &str) -> &str {
    let bytes = str.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &str[0..i];
        }
    }

   &str[..]
}

fn slice(i:&[i32]) -> &[i32]{
    &i[..]
}
