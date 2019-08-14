fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    println!("rect1 is {:?}", rect1.area());
    println!("rect1 is {:?}", Rectangle::square(5));
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数？静态函数？
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}