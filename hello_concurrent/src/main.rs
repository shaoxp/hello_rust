use std::thread;
use std::time::Duration;

use std::sync::mpsc;

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

    handle.join().unwrap();


    for i in 1..5 {
        println!("number {} from main thread",i);
        thread::sleep(Duration::from_millis(1));
    }


    let v = vec![1,2,3];
    let h = thread::spawn(move||{
        println!("{:#?}",v);
    });
    // let a = v.len();

    h.join().unwrap();

    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move||{

        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
            ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
      
    });

    thread::spawn(move||{

        let vals = vec![
            String::from("more"),
            String::from("mesage"),
            String::from("for"),
            String::from("you")
            ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
      
    });
    
    loop{
        match rx.recv(){
            Ok(recieved) =>{
                      println!("r: {}",recieved);
                      // thread::sleep(Duration::from_secs(1))
                    }
            Err(_) => break
        }
    }

    // for received in rx {
    //     println!("Got：{}",received);
    // }
  
  
}
