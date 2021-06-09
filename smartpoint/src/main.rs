fn main() {
    println!("Hello, world!");

    let list = List::Cons(1,Box::new(List::Cons(2,Box::new(List::Cons(3,Box::new(List::Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    let x = String::from("abc");
    let y = &x;
    let z = y.chars().next().unwrap();
    assert_eq!(String::from("abc"),x);
    assert_eq!(String::from("abc"),*y);
    assert_eq!('a',z);

    let x = 5;
    let y = MyBox::new(5);
    let mut y = MyBox::new(x);
    *y = 6;
    let z = &y;

    assert_eq!(5,x);
    assert_eq!(6,*y);
    assert_eq!(6,**z);

    let m = MyBox::new(String::from("ryan"));
    hello(&m);
}

enum List{
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

use std::ops::Deref;
use std::ops::DerefMut;
impl<T> Deref for MyBox<T>{
    type Target  = T;

    fn deref(&self)->&T{
        &self.0
    }
}

impl<T> DerefMut for MyBox<T>{

    fn deref_mut(&mut self) -> &mut T{
        & mut self.0
    }
}

fn hello(name: &str){
    println!("Hello:{}",name);
}
