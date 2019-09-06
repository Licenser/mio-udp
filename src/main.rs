use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use std::thread;
use std::time::Duration;

const IN: Token = Token(0);

fn main() {
    let poll = Poll::new().unwrap();

    let socket = UdpSocket::bind(&format!("{}:{}", "0.0.0.0", 9000).parse().unwrap()).unwrap();
    poll.register(&socket, IN, Ready::readable(), PollOpt::edge())
        .unwrap();
    //poll.register(&socket, IN, Ready::readable(), PollOpt::level()).unwrap();

    let mut events = Events::with_capacity(1024);
    let mut buf = [0; 65535];

    thread::spawn(|| {
        use std::net::UdpSocket;
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
        loop {
            socket.send_to(
                b"Argle bargle, foofaraw, hey diddy ho diddy no one knows",
                ("127.0.0.1", 9000),
            );
        }
    });

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))
            .unwrap();
        for event in events.iter() {
            match event.token() {
                IN => {
                    let mut n = socket.recv(&mut buf).unwrap();
                    while n != 0 {
                        print!(".");
                        n = socket.recv(&mut buf).unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
