use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let y: ImportantExcerpt<'_>;

    {
        let x = "str";
        y = ImportantExcerpt { part: x };
    }

    println!("{:?}", y);
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", first_word("hola"));
}

fn first_word<'a>(r: &'a str) -> &'a str {
    let s = r;
    s
}
