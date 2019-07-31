fn main() {
    let four = IpAddressKind::V4;
    route(four);
    let six = IpAddressKind::V6;
    route(six);

    let ip_address = IpAddress{
        kind: IpAddressKind::V6,
        address: "1.1.1.1".to_string()
    };
    println!("{:?}", ip_address);

    let ip_address = IpAddr::V6("0.0.0.0".to_string());
    println!("{:?}", ip_address);

    let ip_address = IpAddr::V4(0,0,0,0);
    println!("{:?}", ip_address);

    //message
    let message = Message::Quit;
    message.call();

    let message = Message::Move{
        x: 23,
        y: 44
    };
    message.call();

    let message = Message::Write("example".to_string());
    message.call();

    let message = Message::ChangeColor(1,2,3);
    message.call();

    //Option
    let some_number = Some(4);
    println!("{:?}", some_number);
    let some_string = Some("string");
    println!("{:?}", some_string);
    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);

    let some_number_value = some_number.expect("error");
    println!("{}", some_number_value);

    let absent_number_value = absent_number.expect("This absent number is empty");
    println!("{}", absent_number_value);
}

fn route(ip_kind: IpAddressKind) {
    println!("kind: {:?}", ip_kind);
}

#[derive(Debug)]
enum IpAddressKind{
    V4,
    V6
}

#[derive(Debug)]
struct IpAddress{
    kind: IpAddressKind,
    address: String
}

#[derive(Debug)]
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8,u8,u8)
}

impl Message{
    fn call(&self){
        println!("{:?}", self);
    }
}