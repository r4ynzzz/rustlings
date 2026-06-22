#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo("I was here.".to_string()));
    println!("{:?}", Message::ChangeColor(33, 44, 55));
    println!("{:?}", Message::Quit);
}
