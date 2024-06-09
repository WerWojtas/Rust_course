mod max;

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

struct Pair2<T, U> {
    x: T,
    y: U
}

struct Introduction<'a> {
    intro : &'a str
}

impl<'a> Introduction<'a> {
    fn print(&self) {
        println!("{}", self.intro);
    }

    fn returnIntro(&self) -> &'a str {
        self.intro
    }
}

impl <T : PartialOrd + Copy + Clone> Pair<T> {
    fn bigger(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

impl <T: Display + Clone, U: Display + Clone> Pair2<T, U> {
    fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

fn len_longer_array<'a>(a : &'a [i32], b : &'a [i32]) -> &'a [i32] {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn get_sample_text() -> &'static str {
    "Just a sample text"
}

fn main() {
    let array = [1, 2, 3,];
    let array2 = [1, 2, 3, 4, 5];
    let num = len_longer_array(&array, &array2);
    println!("The longer array is: {:?}",  num);
    let text = String::from("Introduction to a long text. The rest of long text with many sentences.");
 
    let intro = text.split('.').next().expect("Could not find a first sentence.");
 
    let i = Introduction { intro };
}
