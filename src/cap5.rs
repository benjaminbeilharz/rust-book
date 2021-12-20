struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User { 
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    // can we use go this way onto a default constructor?
    fn new() -> Self {
        Self {
            width: 50,
            height: 60,
        }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn set_height(&mut self, val: u32) {
        self.height = val;
    }

    fn set_width(&mut self, val: u32) {
        self.width = val;
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn solve() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let mut rect_const = Rectangle::new();
    dbg!(&rect_const);
    rect_const.set_width(10);
    rect_const.set_height(30);
    dbg!(&rect_const);

    dbg!(&rect);
}
