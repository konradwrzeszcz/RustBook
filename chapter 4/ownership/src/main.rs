fn main() {
    string_literals();
    not_string_literals();
    string_type();
    data_move();
    function_data_move();
    return_values();
}

fn string_literals(){
    println!("string literal");
}

fn not_string_literals(){
    let s = "not literal";
    println!("{}",s); // we have to pass value of 's' as not literal string

    let mut s = "mutable not literal";
    let s2 = s.replace("a", "4");    
    println!("{}", s);
    println!("{}", s2);
    s = "another mutable not literal";
    println!("{}", s);
}

fn string_type(){
    let mut s = String::from("string ");
    s.push_str("type");
    println!("{}", s);
}

fn data_move(){
    //move -> shallow copy + old pointer removing
    let s1 = String::from("string type");
    let s2 = s1;
    //s1 pointer from the stack is copied to s2 -> s2 pointer points same data as s1 pointer -> s1 pointer goes out of scope 
    //this operation is known as MOVE
    println!("s2: {}", s2);


    //clone -> deep copy
    let s1 = String::from("string type");
    let s2 = s1.clone();

    println!("s1: {} s2: {}", s1, s2);


    //for data like integer with known size, shallow copy and deep copy is the same operation cause data is on stack (like string pointer)
    //that's why we don't have to call '.clone()' method
    let x = 5;
    let y = x;

    //both values are on the stack
    println!("x: {} y: {}", x, y);
}

fn function_data_move(){
    let x = 4;
    makes_copy(x);
    println!("copied x: {}", x);

    let s = String::from("string type");
    takes_ownership(s);
    //value of s was moved to takes_ownership method
}

fn takes_ownership(some_string: String) {
    println!("took ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("made copy: {}", some_integer);
}

fn return_values() {
    let s1 = gives_ownership();
    println!("get ownership: {}", s1);

    let s2 = takes_and_gives_back(s1);
    println!("get ownership back: {}", s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string
}