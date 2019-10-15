use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    //channel try_recv
    thread::spawn(move || {
        for i in 1..5{
            let val = String::from("hi");
            tx.send(val).unwrap();
            thread::sleep_ms(1000);
        }
    });

    for i in 1..10{
        let received = rx.try_recv();

        match received {
            Ok(v) => println!("Got: {}", v),
            Err(e) => println!("Error: {}", e),
        }

        thread::sleep_ms(500);
    }

    let (tx2, rx2) = mpsc::channel();

    //channel waiting
    let v = vec![1,2,3,4];

    thread::spawn(move || {
        for val in v{
            tx2.send(val.to_string()).unwrap();
            thread::sleep_ms(1000);
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }

    //multiple producers
    let (tx3, rx3) = mpsc::channel();

    let tx4 = mpsc::Sender::clone(&tx3);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx4.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Got: {}", received);
    }
}
