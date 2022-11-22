// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
   Move(i32,i32),
   Echo(String),
   ChangeColor(i32,i32,i32),
   Quit, // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let x;
    let y;
    let messages = [
        Message::Move  (x,y) = (10,30) ,
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
