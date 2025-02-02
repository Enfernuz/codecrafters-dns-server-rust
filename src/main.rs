#[allow(unused_imports)]
use std::net::UdpSocket;

mod dns;
use dns::message::Header;
use dns::message::Message;
use dns::message::Question;
use dns::message::LabelSequence;
use dns::message::Label;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut question = Question::new();
                question.get_name().get_labels().extend([Label::new("codecrafters"), Label::new("io")]);

                let mut header: Header = Header::default();
                header.set_id(1234);
                header.set_qr(true);
                header.set_qd_count(1);
                
                let message = Message::new(header, Some(question));
                let response = message.encode();
                dbg!("Bytes: {}", &response);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
