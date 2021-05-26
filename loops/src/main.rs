fn main() {
    let mut count = 0;
    let mut result = loop {
        count += 1;
        if count == 10{
            break count*2;
        }
    };
   
    println!("result is {}", result);

    while result >10 {
        println!("{}",result);
        result-= 1;
    }

    let a = [10,20,30,40,50];
    let mut i = 0;
    while i < a.len() {
        println!("{}",a[i]);
        i += 1;
    }

    for ai in a.iter() {
        println!("{}",ai);
    }

    for num in 1..10 {
        println!("{}",num);
    }


    for num in (1..10).rev() {
        println!("{}",num);
    }

    println!("fib():{}",fib(50));
}

fn fib(n : u32) -> u64{
    if n==0 { 
        1
    }else if n == 1{
        1
    }else{
        fib(n-1) + fib(n-2)
    }
}
