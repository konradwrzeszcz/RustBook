fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(2, 5);
    another_function3();
    println!("five: {}", five());
    //comment
    println!("six: {}", plus_one(five()));
}

fn another_function(){
    println!("another function");
}

fn another_function2(x: i32, y: i32){
    println!("another function {}", x + y);
}

fn another_function3(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("fun3 x: {}", x);
    println!("fun3 y: {}", y);
}

fn five()-> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}
