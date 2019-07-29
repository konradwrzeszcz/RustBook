fn main() {
    //without references
    let s = String::from("long string");
    let (s, length) = calculate_length_without_references(s);
    println!("Without ref: The length of '{}' is {}", s, length);

    //with references
    //by using '&s' we are passing a pointer to a pointer
    //function get only a pointer to a string '&s', that's why it doesn't have an ownership of string 's'
    //this operation is called BORROWING
    let s = String::from("long string");
    let length = calculate_length_with_references(&s);
    println!("With ref: The length of '{}' is {}", s, length);

    //mutable reference
    //it's possible to have only one mutable reference to a particular data in a scope
    let mut s = String::from("The beginning of a string ");
    mutable_reference(&mut s);
    println!("Mutable ref: {}", s);

    //it's not possible to have mutable and immutable reference to a particular data
    //BUT scope of immutable reference is finished through the last time that reference is used
    let mut s = String::from("immutable string");
    let s1 = &s;
    let s2 = &s;
    println!("Immutable: s1 = {}, s2 = {}, s = {}", s1, s2, s);

    let s3 = &mut s; //string 's' is borrowed
    s3.push_str(" changed");
    println!("Mutable: s3 = {}", s3);
}

fn calculate_length_without_references(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_length_with_references(s: &String) -> usize {
    s.len()
    //we cannot change string 's' inside function
    //reference parameter is immutable
}

fn mutable_reference(s: &mut String){
    s.push_str("-> the end of a string")
}
