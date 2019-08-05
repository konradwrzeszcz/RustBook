use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

//RUST_BACKTRACE=1 cargo run
fn main() {
    const file_name: &str = "hello.txt";
    // open_file_ugly(file_name);
    open_file_more_elegant_way(file_name);

    // File::open("exampleFile.txt").unwrap();
    File::open("exampleFile.txt").expect("Failed to open exampleFile.txt");
    // let f = File::open("hello.txt")?;

    method();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn open_file_ugly(file_name: &str){
    let f = File::open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(created_file) => created_file,
                Err(e) => panic!("Error: {:?}", e)
            },
            other_error => panic!("Other error: {:?}", other_error)
        }
    };
}

fn open_file_more_elegant_way(file_name: &str){
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error|{
                panic!("Error: {:?}", error);
            })
        } else {
            panic!("Other error: {:?}", error);
        }
    });
}

fn method(){
    method2();
}

fn method2(){
    method3();
}


fn method3(){
    method4();
}


fn method4(){
    //panic!("panic");
    let v = vec![1, 2, 3];

    v[99];
}

