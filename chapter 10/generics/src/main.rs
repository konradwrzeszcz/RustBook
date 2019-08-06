//rust use monomorphization during compilation to increase performance of code
//i.e. compiler generate code for every use of generics
fn main() {
    let number_list = vec![0,1,2,3];
    println!("Largest: {}", largest(&number_list));

    let char_list = vec!['a','b','c'];
    println!("Largest: {}", largest(&char_list));

    let point_int = Point{x: 4, y: 1};
    let point_float = Point{x: 4.5, y: 1.2};
    println!("x: {}", point_int.x());
    println!("distance from origin: {}", point_float.distance_from_origin());
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for (i, item) in list.iter().enumerate() {
        if item > largest {
            largest = &list[i];
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T 
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}