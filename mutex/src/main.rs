use std::sync::{Mutex,Arc};
use std::thread;
use std::rc::Rc;
use std::marker::Send;

fn main() {
    println!("Hello, world!");

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m={:?}",m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move||{

            let mut num = counter.lock().unwrap();
            *num+=1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result:{}",*counter.lock().unwrap());
}
