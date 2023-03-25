struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let mut user1 = User{
        username: String::from("umuty"),
        email: String::from("umuty@asistyazilim.com.tr"),
        sign_in_count: 0,
        active: false,
    };

    let username = user1.username;
    user1.username = String::from("umutyalcinn");

    let user2 = build_user(String::from("Onuryalcinn"), String::from("onuryalcinn@hotmail.com"));

    let user3 = User {
        email: String::from("james@mail.co"),
        username: String::from("james"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area of rectangle: {}", rect.area());

    Rectangle::square(10);
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user (username: String, email: String) -> User{
    User{
        username,
        email, 
        sign_in_count: 0,
        active: false,
    }
}
