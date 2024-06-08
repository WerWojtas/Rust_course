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
/* or 
fn some_function<T, U>(t: T, u: U) -> i32
    where T : Display + Clone, 
          U : Debug + Clone
{
    ...
}*/

fn main() {
    let p1 = Pair{x : 5, y : 3};
    println!("Bigger: {}", p1.bigger());
    let p2 = Pair{x : 5f64, y : 3f64};
    println!("Bigger: {}", p2.bigger());
 
    let pf = Pair2{x: 15i32, y : 12.0f64};
    pf.print();
}
