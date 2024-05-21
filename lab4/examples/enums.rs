use UIEvent::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down
}
 
#[derive(Debug)]
enum UIEvent {
    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char)
}

impl UIEvent {
    fn describe(&self) {
        println!("{:?}", self);
    }
}

fn call(event : UIEvent) {
    match event {
        ButtonClicked => println!("Button clicked"), // simple match
        Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
        KeyPressed(ch) => { // whole block
            let up_ch = ch.to_uppercase();
            println!("Key pressed: {}", up_ch);
        }
    }
}

#[derive(Debug)]
enum Message {
    Move(i32, i32),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit
}
 
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

pub fn main() {
    let key_pressed = UIEvent::KeyPressed('b');
    key_pressed.describe();
    let messages = [Message::Move(1, 2), Message::Echo(String::from("hello world!")), Message::ChangeColor(255, 0, 0), Message::Quit];
    for message in &messages {
        message.call();
    }
    let clicked = ButtonClicked;
    let scroll = Scroll(Direction::Down);
    let key_pressed = KeyPressed('b');
    call(clicked);
    call(scroll);
    call(key_pressed);
}