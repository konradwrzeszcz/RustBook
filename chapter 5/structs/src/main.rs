struct User {
    username: String,
    email: String,
    sing_in_count: i64,
    active: bool
}

//tuple struct
struct Color(u8, u8, u8);

//unit type struct
struct Empty();

fn main() {
    let mut user = User{
        username: String::from("username"),
        email:  String::from("email"),
        active: true,
        sing_in_count: 64,
    };

    println!("Username: {}, email: {}, active: {}, sing_in_count: {}", user.username, user.email, user.active, user.sing_in_count);
    user.username = String::from("other username");
    println!("Username: {}, email: {}, active: {}, sing_in_count: {}", user.username, user.email, user.active, user.sing_in_count);

    let user = build_user("name".to_string(), "email address".to_string());
    println!("Username: {}, email: {}, active: {}, sing_in_count: {}", user.username, user.email, user.active, user.sing_in_count);

    let user2 = User{
        username: user.username,
        email: user.email,
        active: false,
        sing_in_count: 12
    };
    println!("Username: {}, email: {}, active: {}, sing_in_count: {}", user2.username, user2.email, user2.active, user2.sing_in_count);

    let user3 = User{
        sing_in_count: 5,
        ..user2
    };
    println!("Username: {}, email: {}, active: {}, sing_in_count: {}", user3.username, user3.email, user3.active, user3.sing_in_count);

    let white = Color(255, 255, 255);
    println!("white: r: {}, g: {}, b: {}", white.0, white.1, white.2);

    let empty = Empty();
}

fn build_user(username: String, email: String) -> User{
    User{
        //if variables and fields have the same name, we haven't to add field name to initialization 
        username,
        email,
        active: true,
        sing_in_count: 1
    }
}