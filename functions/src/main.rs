fn main() {
    println!("Hello, world!");

    another_function(100,10);

    let x = 5;
    let y = {
        let x = 3;
        x+1
    };

    println!("y is {}", y);

    println!("add twice: {}", add_twice(add_one, 5) );

    println!("add twice: {}", add_twice(|x|{x+2}, 5) );
}

fn another_function(x : i32, y : i32){
    println!("Hello from another function! the value is {}" ,x+y)
}


fn add_one(x:i32)->i32{
    x+1
}

fn add_twice(f:fn(x:i32)->i32,arg:i32)->i32{
    f(arg)+f(arg)
}