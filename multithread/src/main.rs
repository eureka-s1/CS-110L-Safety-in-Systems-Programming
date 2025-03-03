use std::rc::Rc;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // ownership of val is moved to the other thread
        // println!("val is {}", val); 

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // handle.join().unwrap();
    // let received = rx.try_recv().unwrap();
    
    let received = rx.recv().unwrap();

    println!("Got: {}", received);

    //加入这一行会使得管道内消息堆积？
    // handle.join().unwrap(); 


    for receive in rx {
        println!("Got: {}", receive);
    }



    // Mutex: single thread
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // Mutex: multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("num = {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}