fn main() {
    let mut s = "foo".to_string();
    let s2 = "bar";
    s.push_str(s2);
    println!("s: {}", s);
    println!("s2: {}", s2);

    s.push('2');
    println!("s: {}", s);

    let s1 = "first ".to_string();
    let s2 = "second".to_string();
    let s3 = s1 + &s2;

    //s1 is borrowed
    //println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s: {}", s);

    let slice = &s[0..3];
    println!("slice: {}", slice);

    let hello = "Здравствуйте";
    let slice = &hello[0..4];
    println!("slice: {}", slice);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    //strings in rust are stored as UTF-8 byte array
    //we can get value of string in three different ways: as bytes, as scalar value and as grapheme clusters
    //Hindi letters: नमस्ते
    //bytes -> [224,...,135]
    //scalar values -> [न, म, स, , त, ]
    //grapheme clusters -> [न, म, स, त] - this functionality is not provided by standard library, because it's complex
}
