use rc_cycle::List::*;
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    println!("Hello, world!");

    let a = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));

    let b = Rc::new(Cons(6,RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("{}",Rc::strong_count(&a));
    println!("{}",Rc::strong_count(&b));

    // println!("{:?}",a.tail());
}
