use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let lenght = get_lenght();
    let password = generate(lenght);
    println!("Generated password: {:?} of lenght {:?}", password, lenght)
 
}

fn get_lenght() -> i32{
    let mut lenght : String;
    loop {
        println!("Type the length of the password:");
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);
        let mut i = 0;
        lenght = String::new();
        let mut flag = false;
        loop{
            if let Some(value) = command.chars().nth(i).unwrap().to_digit(10) {
                lenght.push(std::char::from_digit(value, 10).unwrap());
                flag = true;
                i+=1;
            }
            else{
                break;
            }
        }
        if flag {
            break;
        }
        println!("Invalid input, please type a number");
    }
    lenght.parse::<i32>().unwrap()

}

fn generate(length: i32) -> String {
    let signs = [33,35, 36, 37, 38, 63, 64];
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        match rng.gen_range(0..3){
            0 => password.push(rng.gen_range(48..58) as u8 as char), // 48..58 is the range of ASCII numbers for digits
            1 => password.push(rng.gen_range(65..91) as u8 as char), // 65..91 is the range of ASCII numbers for uppercase letters
            2 => password.push(rng.gen_range(97..123) as u8 as char), // 97..123 is the range of ASCII numbers for lowercase letters
            _ => password.push(signs[rng.gen_range(0..7)] as u8 as char), // 33, 35, 36, 37, 38, 63, 64 are the ASCII numbers for special characters
        }
    }
    password
}