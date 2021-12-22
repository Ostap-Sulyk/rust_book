#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn build_rectangle(width: usize, height: usize) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::build_rectangle(10, 15);
    let rect2 = Rectangle::build_rectangle(10, 15);

    println!("{}", rect1.can_hold(&rect2));
}
