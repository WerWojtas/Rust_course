

fn main() {
    let increment_v1 = |x: u32| -> u32 { x + 1 };
    let increment_v2 = |x| { x + 1 };
    let increment_v3 = |x| x + 1; // only if there is only one expression
    let y = 5;
    let result = increment_v1(y);
    println!("The result is: {}", result);
    let result = increment_v2(y);
    println!("The result is: {}", result);
    let result = increment_v3(y);
    println!("The result is: {}", result);
    let mut x = 5;
    let mul_5 = |a| a * x;
    println!("The result is: {}", mul_5(2));
    let mut x = 5;
    let mut add_to_x = |a| x += a;
    add_to_x(2);
    println!("The result is: {}", x);

    let mut x = vec![1, 2, 3];
    let equal_to_x = move |y| y == x; 
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    let first = iter.next();
    println!("{:?}", first);
    let second = iter.next();
    println!("{:?}", second);
    let third = iter.next();
    println!("{:?}", third);
    let fourth = iter.next();
    println!("{:?}", fourth);

    let mut v = vec![1, 2, 3];
 
    v.iter_mut().map(|x| *x +=1).collect::<Vec<_>>();
    println!("{:?}", v);
    let v = vec![1, 2, 3];
    let v2 = v.into_iter().map(|x | x * 2).collect::<Vec<_>>();
    println!("{:?}", v2);
}