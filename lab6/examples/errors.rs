use std::fs;
use std::io;
use anyhow::{anyhow, bail, Result};

const DEFAULT_WIDTH: f64 = 1.0;
const DEFAULT_HEIGHT: f64 = 1.0;


#[derive(Debug)]
struct Rectangle {
    width : f64,
    height : f64
}
 
impl Rectangle {
    fn new(width : f64, height : f64) -> Rectangle {
        if width < 0.0 || height < 0.0 {
            panic!("Rectangle::new() called with negative values");
        }
        Rectangle { width, height }
    }

    fn new_option(width : f64, height : f64) -> Option<Rectangle> {
        if width < 0.0 || height < 0.0 {
            None
        } else {
            Some(Rectangle { width, height })
        }
    }

    fn new_result(width : f64, height : f64) -> Result<Rectangle, String> {
        if width < 0.0 || height < 0.0 {
            Err("Rectangle::new() called with negative values".to_string())
        } else {
            Ok(Rectangle { width, height })
        }
    }

    fn new_anyhow(width : f64, height : f64) -> Result<Rectangle> {
        if width < 0.0 || height < 0.0 {
            anyhow::bail!("Rectangle::new() called with negative values");
        }
        Ok(Rectangle { width, height })
    }
 
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn read_from_file(path : &str) -> Result<Rectangle> {
        let s = fs::read_to_string(path)?;
        let mut iter = s.split_whitespace();
        let width : f64 = iter.next().ok_or(anyhow!("Cannot convert string to width"))?.parse()?;
        let height : f64 = iter.next().ok_or(anyhow!("Cannot convert string to height"))?.parse()?;
 
        Ok(Rectangle::new_anyhow(width, height)?)
 
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle{ width : DEFAULT_WIDTH, height : DEFAULT_HEIGHT }
    }
}

fn bigger(w1: f64, h1 : f64, w2 : f64, h2 : f64) -> Result<Rectangle, String> {
    let r1 = Rectangle::new_result(w1, h1);
    let r2 = Rectangle::new_result(w2, h2);
 
    match (r1, r2) {
        (Ok(rec1), Ok(rec2)) => {
            if rec1.area() > rec2.area() {
                Ok(rec1)
            } else {
                Ok(rec2)
            }
        },
        _ => Err("Rectangle cannot have negative width or height".to_string()),
    }
}

fn bigger_q(w1: f64, h1 : f64, w2 : f64, h2 : f64) -> Result<Rectangle, String> {
    let r1 = Rectangle::new_result(w1, h1)?;
    let r2 = Rectangle::new_result(w2, h2)?;
 
    if r1.area() > r2.area() {
        Ok(r1)
    } else {
        Ok(r2)
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_new_rectangle() {
        // given
        let width = 4.5;
        let height = 5.7;
 
        // when
        let r = Rectangle::new(width, height);
 
        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_new_rectangle_with_negative() {
        Rectangle::new(-1.0, 1.0);
    }

    #[test]
    fn test_new_rectangle_option() {
        let width = 4.5;
        let height = 5.7;
        let r = Rectangle::new_option(width, height);
        assert!(r.is_some());
        let r = r.unwrap();
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
        let width = -4.5;
        let r = Rectangle::new_option(width, height);
        assert!(r.is_none());
    }

    #[test]
    fn test_new_rectangle_result() {
        let width = 4.5;
        let height = 5.7;
        let r = Rectangle::new_result(width, height);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
        let width = -4.5;
        let r = Rectangle::new_result(width, height);
        assert!(r.is_err());
        assert_eq!(r.err().unwrap(), "Rectangle::new() called with negative values");
    }

    #[test]
    #[should_panic]
    fn test_unwrap() {
        // that is ok
        let r = Rectangle::new_result(1.0, 2.0);
        let rec = r.unwrap(); // consumes the value
 
        // that generates panic
        let r = Rectangle::new_result(-1.0, 2.0);
        let rec = r.unwrap(); // consumes the value
 
    }
    #[test]
    #[should_panic]
    fn test_expect() {
        // that is ok
        let r = Rectangle::new_result(1.0, 2.0);
        let rec = r.expect("This should be always a proper rectangle");
    
        // that generates panic
        let r = Rectangle::new_result(1.0, -2.0);
        let rec = r.expect("Don't do that!");
    }

    #[test]
    fn test_unwrap_or_else() {
        let r = Rectangle::new_result(-1.0, -2.0);
        let rec = r.unwrap_or_else(|_| Rectangle::new_result(DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap());
    
        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unwrap_or_default() {
        let r = Rectangle::new_result(-1.0, -2.0);
        let rec = r.unwrap_or_default();
    
        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);
    
        let r = Rectangle::new_result(1.0, 2.0);
        let rec = r.unwrap_or_default();
        assert!((rec.width - 1.0).abs() < f64::EPSILON && (rec.height - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bigger() {
        // given
        let w1 = 1.0;
        let h1 = 2.0;
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = bigger(w1, h1, w2, h2);
    
        // then
        assert!((r.unwrap().area() - (w2 * h2)).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_bigger_err() {
        // given
        let w1 = 1.0;
        let h1 = -2.0; // wrong width
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = bigger(w1, h1, w2, h2);
    
        // then
        assert!(r.is_err());
    }

    #[test]
    fn test_biggerQ() {
        // given
        let w1 = 1.0;
        let h1 = 2.0;
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = bigger_q(w1, h1, w2, h2);
    
        // then
        assert!((r.unwrap().area() - (w2 * h2)).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_bigger_q_err() {
        // given
        let w1 = 1.0;
        let h1 = -2.0; // wrong width
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = bigger_q(w1, h1, w2, h2);
    
        // then
        println!("{:?}", r);
        assert!(r.is_err());
    }

    #[test]
    fn test_read_from_not_existing_file() {
        let path = "not_existing_file.txt";
        let r = Rectangle::read_from_file(path);
        assert!(r.is_err());
    }
}
 
fn main() {
    // nothing is here
}