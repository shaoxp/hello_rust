use crate::List::*;
use crate::List2::*;

fn main() {
    println!("Hello, world!");
    let a = Rc::new(RefCell::new(Cons(5,Rc::new(RefCell::new(Cons(20,Rc::new(RefCell::new(Nil))))))));
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


    let v = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons2(Rc::clone(&v),Rc::new(Nil2)));
    let b = Cons2(Rc::new(RefCell::new(3)),Rc::clone(&a));
    println!("a: {:?}",a);
    println!("b: {:?}",b);

    *v.borrow_mut()+=10;
    
    println!("a: {:?}",a);
    println!("b: {:?}",b);
}
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
    Cons(i32,Rc<RefCell<List>>),
    Nil,
}

#[derive(Debug)]
enum List2{
    Cons2(Rc<RefCell<i32>>,Rc<List2>),
    Nil2,
}

