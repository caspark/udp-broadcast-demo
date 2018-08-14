use std::env;
use std::net::UdpSocket;
use std::str;

const LISTEN_PORT: u16 = 8900;
const SRC_PORT: u16 = 8900;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let dest_addr = args[1].clone();
        let data_to_send = args[2].clone();

        let src_addr = format!("0.0.0.0:{}", SRC_PORT);
        println!("binding to {} for sending", src_addr.as_str());
        let socket = UdpSocket::bind(src_addr).expect("bind should succeed");

        println!("broadcasting to {} data of {}", dest_addr.as_str(), data_to_send.as_str());
        socket
            .send_to(data_to_send.as_str().as_bytes(), format!("{}:{}", dest_addr.as_str(), LISTEN_PORT))
            .expect("couldn't send data");
    } else {
        let listen_addr = format!("0.0.0.0:{}", LISTEN_PORT);
        println!("listening on {}...", listen_addr.as_str());
        let socket = UdpSocket::bind(listen_addr.as_str()).expect("bind should succeed");

        let mut buf = [0; 100];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("read should succeed");
        println!("read {} bytes from {:?}", number_of_bytes, &src_addr);

        let filled_buf = &mut buf[..number_of_bytes];
        match str::from_utf8(filled_buf) {
            Ok(s) => println!("bytes are valid UTF8; string: {}", s),
            Err(_) => println!("bytes are not UTF8, raw bytes: {:?}", filled_buf),
        }
    }
}
