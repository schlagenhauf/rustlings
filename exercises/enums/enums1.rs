// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Echo,
    Move,
    Quit,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
