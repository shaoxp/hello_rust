use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawn thread",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from main thread",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join();


    for i in 1..5 {
        println!("number {} from main thread",i);
        thread::sleep(Duration::from_millis(1));
    }


    let v = vec![1,2,3];
    let h = thread::spawn(move||{
        println!("{:#?}",v);
    });
    let a = v.len();

    h.join().unwrap();
}
