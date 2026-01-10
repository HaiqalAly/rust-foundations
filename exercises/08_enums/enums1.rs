#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize = 12,
    Move = 15,
    Echo = 13,
    ChangeColor = 20,
    Quit = 22
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
