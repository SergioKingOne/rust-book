#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        if rect.area() <= self.area() {
            true
        } else {
            false
        }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            //
        }
    }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    let msg1 = Message::ChangeColor((1), (2), (3));
    let msg2 = ChangeColorMessage(1, 2, 3);

    println!("{:?}, {}", msg1, msg2.0);
    // let home = IpAddr {
    //     kind: IpAddrKind::V4(255, 0, 1, 4),
    //     address: String::from("127.0.1.2"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // let v4 = IpAddrKind::V4;
    // let v6 = IpAddrKind::V6;

    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // let rect2 = Rectangle {
    //     width: 30 * 3,
    //     height: 50,
    // };

    // let sq = Rectangle::can_hold(&rect1, &rect2);

    // print!("{}", sq);

    // println!("Does rect2 fit in rect1?: {}", rect1.can_hold(&rect2));

    // dbg!(&rect1);

    // println!("{:#?}", rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     rect1.area()
    // );

    // let rect2 = Rectangle {
    //     width: 30,
    //     height: 50,
    //     name: String::from("Hola"),
    // };
}

// fn area(rect1: &Rectangle) -> u32 {
//     rect1.width * rect1.height
// }
