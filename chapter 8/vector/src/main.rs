fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(6);
    print_vec(&v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    print_vec(&v);

    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(el) => println!("The hundredth element is {}", el),
        None => println!("There is no hundredth element."),
    }
    //will panic
    //let does_not_exist = &v[100];

    //vector with multiple types
    let v = vec![
        VectorWithMultipleTypes::Int(10),
        VectorWithMultipleTypes::Double(5.3),
        VectorWithMultipleTypes::Text("sample text".to_string())
    ];
    print_vec(&v);
}

fn print_vec<T: std::fmt::Debug>(v: &Vec<T>){
    for val in v{
        println!("{:?}", val);
    }
}

#[derive(Debug)]
enum VectorWithMultipleTypes{
    Int(i32),
    Double(f64),
    Text(String)
}