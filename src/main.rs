mod front_of_house;

fn eat_at_restaurant() {
    front_of_house::welcome();
}
fn main() {
    eat_at_restaurant();

    front_of_house::hosting::add_to_waitlist();

    let mut v = vec![1, 2, 3, 4];

    v.push(5);

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third = v.get(2);

    match third {
        Some(third) => println!("Your third element is {third}"),
        None => println!("There's no such element"),
    }

    println!("\nUsing an Enum to Store Multiple Types");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hola")),
        SpreadsheetCell::Float(23.54),
    ];

    for i in row {
        println!("{:?}", i);
    }
}
