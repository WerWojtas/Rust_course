use std::{fmt::Formatter, iter::Rev};
use std::fmt::Display;


#[derive(Debug)]
struct Rectangle {
    x : f32,
    y : f32
}

impl Rectangle{
    fn factor(&mut self, factor: f32){
        self.x = self.x*factor;
        self.y = self.y*factor
    }

    fn new_square(x:f32) -> Rectangle{
        Rectangle{x,y : x}
    }
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn describe(&self) {
        println!("I'm a general shape.");
    }

}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x*self.y
    }
    fn perimeter(&self) -> f32 {
        self.x*2.0 + self.y*2.0
    }
    
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "I am rectangle, my width is {} and my height is {}", self.x, self.y)
    }
}

pub fn main() {
    let r = Rectangle{x: 1.0, y: 2.0};
    println!{"{:?}",r};
    println!("x: {}, y: {}", r.x, r.y);
    let r2 = Rectangle{x: 3.0, ..r};
    println!("x: {}, y: {}", r2.x, r2.y);
    let mut r3 = Rectangle{x: 3.0, y: 5.0};
    println!("[{},{}]", r3.x, r3.y);
    r3.x = 7.0;
    println!("[{},{}]", r3.x, r3.y);
    let area = r3.area();
    println!("Area of {:?} r3 is {}",r3, area);
    let ob = r3.perimeter();
    println!("Perimeter of {:?} r3 is {}",r3, ob);
    r3.factor(6.0);
    println!("R3 is now {:?}",r3);
    let square = Rectangle::new_square(6.0);
    println!("My square is {:?}", square);
    square.describe();
    println!("{}", r3)
}