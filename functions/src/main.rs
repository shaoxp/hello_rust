fn main() {
    println!("Hello, world!");

    another_function(100,10);

    let x = 5;
    let y = {
        let x = 3;
        x+1
    };

    println!("y is {}", y);
}

fn another_function(x : i32, y : i32){
    println!("Hello from another function! the value is {}" ,x+y)
}
