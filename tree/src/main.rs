use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    println!("Hello, world!");

    let leaf = Rc::new(Node{
        v:3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf: rc:{},weak:{}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node{
            v:5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
        println!("leaf parent:{:?}",leaf.parent.borrow().upgrade());
    
        println!("leaf: rc:{},weak:{}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
        println!("branch: rc:{},weak:{}",Rc::strong_count(&branch),Rc::weak_count(&branch));
    }

    println!("leaf: rc:{},weak:{}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    println!("leaf parent:{:?}",leaf.parent.borrow().upgrade()); 
}



#[derive(Debug)]
struct Node{
    v: i32,
    children : RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}