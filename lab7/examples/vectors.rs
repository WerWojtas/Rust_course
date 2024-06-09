



fn main() {
    let v: Vec<i32> = Vec::new(); // new empty vector
    let mut v = vec![1, 2, 3]; // new vector with values
    let e1 : &i32 = &v[1]; // will panic if out of bounds
    let e2 = v.get(2); // will return None if out of bounds (returns Option)
    println!("{:?}", e1);
    println!("{:?}", e2);
    v.push(4);
    v.push(5);
    let e3 = v.pop(); // removes the last element and returns it
    println!("{:?}", e3);
    // println!("{:?}", e1); will not compile, because e1 is a reference to an element that was removed
    let v = vec![1, 2, 3];
 
    for e in v { 
        println!("{}", e); 
    }
    // println!("{:?}", v); will not compile, because v was moved

    let v = vec![1, 2, 3];
 
    for e in &v { // need to use reference (otherwise the v value is moved)
        println!("{}", *e); // need to explicitly derefer the value
    }

    let mut v = vec![1, 2, 3];
 
    for e in &mut v { // mutable borrow
        *e *= 2; // remember to use dereference operator
    }
    println!("{:?}", v);
    
}
