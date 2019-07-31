fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three")
    }
    else{
        println!("not three")
    }
}
