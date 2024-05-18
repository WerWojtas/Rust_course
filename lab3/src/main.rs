fn main() {
    let mut s = Ship{x: 1.0, y: 1.0, direction: 2.0};
    println!("Ship1 direction: {}", s.direction);
    s.rotate(371);
    println!("Ship1 direction after change: {}", s.direction);
    let mut s = Ship{x: 1.0, y: 1.0, direction: 45.0/180.0*PI};
    s.move_ship(2.0*2f64.sqrt());
    s.position();
    let s2 = Ship{x: 3.0, y: 9.0, direction: 45.0/180.0*PI};
    println!("Distance from ship1 to (3,9): {}", s.calculate_distance(3.0, 9.0));
    println!("Distance between ship1 and ship2: {}", s.calculate_berween(&s2));
}


use std::f64::consts::PI;

struct Ship {
    x : f64,
    y : f64,
    direction : f64,
}

impl Ship {
    fn rotate(&mut self, angle : i32) {
        let radians = angle as f64 * PI / 180.0;
        self.direction = (self.direction + radians) % (std::f64::consts::PI * 2.0)
    }

    fn move_ship(&mut self, distance: f64){
        let delta_x = distance * self.direction.cos();
        let delta_y = distance * self.direction.sin();
        self.x += delta_x;
        self.y += delta_y;

    }

    fn position(&self) {
        println!("x: {:.4}, y: {:.4}", self.x, self.y);
    }

    fn calculate_distance(&self, x : f64, y : f64) -> f64 {
        ((self.x-x).powf(2.0) + (self.y-y).powf(2.0)).sqrt()
    }

    fn calculate_berween(&self, other : &Ship) -> f64 {
        ((self.x-other.x).powf(2.0) + (self.y-other.y).powf(2.0)).sqrt()
    }
}
