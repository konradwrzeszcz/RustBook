fn main() {

    let first_word = first_word("hello world");

    println!("{}", first_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes{
        if item == b' '{
            return i;
        }
    }

    s.len()
}
