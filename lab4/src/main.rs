mod test_pattern;
mod options;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
   let y: Option<Point> = Some(Point { x: 100, y: 200 });
 
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; 
}
