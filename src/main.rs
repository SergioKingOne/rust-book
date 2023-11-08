// // #![allow(unused)]
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     California,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
use crate::garden::vegetables::Brocoli;
pub mod garden;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Order {
        cheese,
        rise,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from(""),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please!", meal.toast);
}

fn main() {
    eat_at_restaurant();

    // ***Defining Modules to Control Scope and Privacy***

    // ***Defining Modules to Control Scope and Privacy***

    // ***Concise Control Flow with if let***
    // let coin = Coin::Penny;
    // let mut count = 0;

    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }

    // let config_max = Some(3u8);

    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }

    // Boilerplate code
    // match config_max {
    //     Some(max) => println!("The max configured value is: {}", max),
    //     _ => (),
    // }

    // ***Concise Control Flow with if let***

    // let dice_roll = 9;

    // match dice_roll {
    //     7 => lose_fancy_hat(),
    //     3 => gains_fancy_hat(),
    //     _ => (),
    //     // other => move_player(other),
    // };

    // fn lose_fancy_hat() {}
    // fn gains_fancy_hat() {}
    // fn reroll() {}
    // fn move_player(x: i32) {}
    // fn plus_one(op: Option<i32>) -> Option<i32> {
    //     match op {
    //         None => None,
    //         Some(x) => Some(x + 1),
    //     }
    // }

    // let five = Some(5);
    // let six = plus_one(five);
    // println!("{:?}", five);

    // let none = plus_one(None);
    // let coin = Coin::Quarter(UsState::Alabama);

    // let coin = value_in_cents(coin);

    // println!("The value of your coin is {}", &coin);

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = match y {
    //     Some(value) => x + value,
    //     None => x, // Provide a default value or handle the None case as appropriate.
    // };

    // println!("The sum is: {}", sum);

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         //
    //     }
    // }

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // let msg1 = Message::ChangeColor((1), (2), (3));
    // let msg2 = ChangeColorMessage(1, 2, 3);

    // println!("{:?}, {}", msg1, msg2.0);
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

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn width(&self) -> bool {
//         self.width > 0
//     }

//     fn can_hold(&self, rect: &Rectangle) -> bool {
//         if rect.area() <= self.area() {
//             true
//         } else {
//             false
//         }
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(us) => 25,
//     }
// }
