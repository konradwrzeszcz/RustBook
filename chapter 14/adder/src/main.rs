use add_one_konwrz20;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {} and plus two is {}!",
        num,
        add_one_konwrz20::add_one(num),
        add_two::add_two(num)
    );
}
