//use std::net::TcpStream;

#[derive(Debug)]
pub enum Pokersuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

// use case
// pub enum Websockect {
//     Tcp(Websocket<TcpStream>),
//     Tls(Websockect<native_tls::TlsStream<TcpStream>>),
// }

pub struct PokerCard {
    pub suit: Pokersuit,
    pub value: u8,
}

pub fn print_card(card: PokerCard) {
    println!("{:?}:{:?}", card.suit, card.value);
}

// fn new(stream: TcpStream) {
//     let mut s = stream;
//     if tls {
//         s = negotiate_tls(stream)
//     }
//     websockect = Websockect::Tls((Websockect));
// }
pub fn plus(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(i), Some(j)) => Some(i + j),
        _ => None,
    }
}
