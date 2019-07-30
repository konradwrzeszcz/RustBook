fn main() {
    let width = 5;
    let height = 10;
    let area = area1(width, height);
    println!("Area1: {}", area);

    let area = area2((width, height));
    println!("Area2: {}", area);

    let rectangle = Rectangle{
        width,
        height
    };
    let area = area3(&rectangle);
    println!("Rectangle: {:?}", rectangle);
    println!("Area3: {}", area);

    let area = rectangle.area();
    println!("Rectangle: {:?}", rectangle);
    println!("Area3: {}", area);


    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(4);
    println!("Square: {:?}", square);
}

fn area1(width: i32, height: i32) -> i32 {
    width*height
}

fn area2(dimensions: (i32, i32)) -> i32 {
    dimensions.0*dimensions.1
}

#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32
}

impl Rectangle{
    fn area(&self) -> i32{
        self.width*self.height
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool{
        self.width > other_rec.width && self.height > other_rec.height
    }
}

//we can use multiple impl blocks -> probably same as extension methods from c#
impl Rectangle{
    //associated function = static function from c#
    fn square(size: i32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn area3(r: &Rectangle) -> i32 {
    r.width*r.height
}