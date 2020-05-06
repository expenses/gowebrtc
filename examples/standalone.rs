use std::io::{Read, Write};

fn main() {
    let listen = std::env::args().nth(1).is_some();
    
    let multiaddr = "/ip4/127.0.0.1/tcp/9091/http/p2p-webrtc-direct".parse().unwrap();

    let transport = gowebrtc::Transport::new();

    if listen {
        let listener = transport.listen(multiaddr).unwrap();
        println!("[listener] Listening");

        loop {
            let connection = listener.accept().unwrap();
            async_std::task::spawn(async move {
                println!("[listener] Got connection");

                loop {
                    let mut stream = connection.accept_stream().unwrap();
                    println!("[listener] Got stream");
                    let mut string = String::new();
                    stream.read_to_string(&mut string).unwrap();
                    println!("[listener] Received:");
                    println!("{}", string);
                    /*});
                    async_std::task::spawn(async move {

                        let bytes = stream.read().unwrap();
                        println!("[listener] Received:");
                        println!("!!{:?}", bytes);
                    });*/
                }
            });
        }
    } else {
        let connection = transport.dial(multiaddr, "peerA").unwrap();
        println!("[dialer] Opened connection");
        let mut stream = connection.open_stream().unwrap();
        println!("[dialer] Opened stream");
        let written = stream.write(b"hey, how is it going. I am the dialer").unwrap();
        println!("{}", written);
    }
}
