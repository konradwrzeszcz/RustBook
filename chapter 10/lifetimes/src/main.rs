fn main() {
    let x = 5;
    let r = &x;
    println!("r {} x {}", r, x);

    println!("longest {}", longest("abc", "abcd"));
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len(){
        s1
    } else {
        s2
    }
}