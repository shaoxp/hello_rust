use crate::List::*;

fn main() {
    println!("Hello, world!");
    let a = Rc::new(Cons(5,Rc::new(Cons(20,Rc::new(Nil)))));
    println!("count:{}",Rc::strong_count(&a));
    let b = Cons(3,a.clone());
    println!("count:{}",Rc::strong_count(&a));
    {
        let c = Cons(1,Rc::clone(&a));
        println!("count:{}",Rc::strong_count(&a));
    }
    drop(b);
    println!("count:{}",Rc::strong_count(&a));

    let x = Rc::new(5);
    let y = Rc::clone(&x);
    let z = Rc::clone(&y);
    let mut zz = *z;

    zz+=1;
    println!("{},{},{}",x,y,zz);
}
use std::rc::Rc;
enum List{
    Cons(i32,Rc<List>),
    Nil,
}


