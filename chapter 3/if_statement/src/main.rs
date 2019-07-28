fn main() {
    let number = 7;

    if number < 5 {
        println!("Hello, world!");
    }
    else if number > 8 {
        println!("something else")
    }
    else{
        println!("Goodbye, world!");
    }

    if_in_let();
    for_loop();
    for_range();
    fibonacci();
}

fn if_in_let(){
    let x = if false{
        5
    } else {
        6
    };

    println!("{}", x);
}

fn for_loop(){
    let arr = [1,2,3,4];

    for item in arr.iter(){
        println!("{}", item);
    }
}

fn for_range()
{
    for i in 1..5{
        println!("item: {}", i);
    }
}

fn fibonacci(){
    let mut first = 1;
    let mut second = 1;
    loop{
        let temp = second;
        second = first + second;
        first = temp;
        println!("fib: {}", first);

        if(first > 1000){
            break;
        }
    }
}