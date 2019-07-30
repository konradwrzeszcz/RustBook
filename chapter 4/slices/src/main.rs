fn main() {
    let s = String::from("hello world");
    let index = ending_index_of_first_word(&s);
    //if string s will change, you can make a mistake by using 'index' variable, because 'index' is not connected/related with 's'
    println!("{}", index);

    //slice is a reference which is storing a pointer to first element of slice [0] and length of whole slice [6-0-1 = 5]
    let slice = &s[0..6];
    println!("slice: {}", slice);
    let slice = &s[..6];
    println!("slice: {}", slice);
    let slice = &s[6..];
    println!("slice: {}", slice);

    let first_word = first_word(&s);
    println!("first word: {}", first_word);

    //string literal is a slice
    let s = String::from("hello world");
    let slice = string_literal_is_a_slice(&s[..]);
    println!("string literal is a slice: {}", slice);

    let s = "hello world";
    let slice = string_literal_is_a_slice(&s[..]);
    println!("string literal is a slice: {}", slice);
    let slice = string_literal_is_a_slice(s);
    println!("string literal is a slice: {}", slice);

    let a = [1, 2, 3, 4];
    let other_slice = &a[1..3];
    println!("number slice: {:?}", other_slice);

}

fn ending_index_of_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

fn string_literal_is_a_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}
