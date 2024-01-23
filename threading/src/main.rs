use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let produce_num = 10;
    let handle = thread::spawn(move || {
        for i in 0..produce_num {
            let val = String::from("hi") + &i.to_string();
            println!("Produced: {val}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle1 = thread::spawn(move || {
        for i in 0..produce_num {
            let val = String::from("hi") + &i.to_string();
            println!("Produced: {val}");
            
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for _ in 0..2 * produce_num {
        let received = rx.recv().unwrap();
        println!("Consumed: {}", received);
    }
    
    handle.join().unwrap();
    handle1.join().unwrap();
}
