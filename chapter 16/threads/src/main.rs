use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Other thread! {:?}", thread::current().id());
            println!("V: {:?}", v);
            thread::sleep_ms(1000);
        }
    });
    
    for i in 1..5 {
        println!("Main thread!{:?}", thread::current().id());
        thread::sleep_ms(1000);
    }
    
    handle.join().unwrap();

    println!("The end");
}
