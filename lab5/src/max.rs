use std::ops::Add;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
pub enum Types {
    Num(i32),
    Float(f32),
    Usize(u32)
}

struct Pair<T> {
    x: T,
    y: T
}

struct Pair2<T, U> {
    x: T,
    y: U
}

impl Into<f32> for Types {
    fn into(self) -> f32 {
        match self {
            Types::Num(n) => n as f32,
            Types::Float(f) => f,
            Types::Usize(u) => u as f32,
        }
    }
}

trait Convert {
    fn convert(&self) -> f32;
}

impl Convert for Types {
    fn convert(&self) -> f32 {
        (*self).into()
    }
}

impl Convert for i32 {
    fn convert(&self) -> f32 {
        return *self as f32;
    }
}

impl Convert for f32 {
    fn convert(&self) -> f32 {
        return *self;
    }
}

impl Convert for u32 {
    fn convert(&self) -> f32 {
        return *self as f32;
    }
}

pub fn max<T>(list: &[T]) -> Option<T>
    where 
        T: PartialOrd + Copy + Clone
    {
        if list.is_empty() {
            return None;
        }
        let mut max = &list[0];
        for i in list {
            if *i > *max {
                max = i;
            }
        }
        Some(*max)
    }

fn mean<T>(list: &[T]) -> Option<f32>
    where 
        T: PartialOrd + Copy + Clone + Convert
    {
        if list.is_empty() {
            return None;
        }
        let mut sum : f32 = 0.0;
        for &i in list {
            sum += i.convert();
        }
        return Some(sum / list.len() as f32);
    }


trait MyAdd<T = Self> {
    fn add(&mut self, other: &T) -> &T;
}
    
impl<T> MyAdd<Pair<T>> for Pair<T> where
    T: Add<Output = T> + Copy
    {
    fn add(&mut self, other : &Pair<T>) -> &Pair<T> {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
        
    }
}

impl<T, U> MyAdd<Pair2<T, U>> for Pair2<T, U> where
    T: Add<Output = T> + Copy,
    U: Add<Output = U> + Copy
    {
    fn add(&mut self, other : &Pair2<T, U>) -> &Pair2<T, U> {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let array = [Types::Num(3), Types::Float(4.0), Types::Usize(5)];
        let max1 = max(&array);
        assert_eq!(max1, Some(Types::Usize(5)));
        let array2 = [15, 12, 5];
        let max2 = max(&array2);
        assert_eq!(max2, Some(15));
        let array3 = [15.0, 12.0, 5.0];
        let max3 = max(&array3);
        assert_eq!(max3, Some(15.0));
        let array4: [Types; 0] = [];
        let max4 = max(&array4);
        assert_eq!(max4, None);
    }

    #[test]
    fn test_mean() {
        let array = [Types::Num(3), Types::Float(4.0), Types::Usize(5)];
        let mean1 = mean(&array);
        assert_eq!(mean1, Some(4.0));
        let array2 = [15, 12, 5];
        let x = 3i32;
        let y = 4.0f32;
        let mean2 = mean(&array2);
        assert_eq!(mean2, Some(10.666667));
        let array3 = [15.0, 12.0, 5.0];
        let mean3 = mean(&array3);
        assert_eq!(mean3, Some(10.666667));
        let array4: [Types; 0] = [];
        let mean4 = mean(&array4);
        assert_eq!(mean4, None);
    }

    #[test]
    fn test_add() {
        let mut p1 = Pair{x : 5, y : 3};
        let p2 = Pair{x : 5, y : 3};
        p1.add(&p2);
        assert_eq!(p1.x, 10);
        assert_eq!(p1.y, 6);
        let mut p3 = Pair2{x : 5, y : 3.0};
        let p4 = Pair2{x : 5, y : 3.0};
        p3.add(&p4);
        assert_eq!(p3.x, 10);
        assert_eq!(p3.y, 6.0);
    }
}