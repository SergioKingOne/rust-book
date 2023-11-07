#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn printing(&self) {
        println!("{}", self.name);
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
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
        name: String::from("Hola"),
    };

    let rect2 = Rectangle {
        width: 30 * 3,
        height: 50,
        name: String::from("Hola"),
    };

    println!("Does rect2 fit in rect1?: {}", rect1.can_hold(&rect2));

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
