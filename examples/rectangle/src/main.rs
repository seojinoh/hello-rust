struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn new(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

fn main() {
    // 방법 1:
    // let mut rect = Rectangle { width: 10, height: 5 };
    // 방법 2:
    let mut rect = new(10, 5);
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
