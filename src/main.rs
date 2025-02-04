#[allow(unused_imports)]
use std::net::UdpSocket;

mod dns;
use dns::message::Answer;
use dns::message::Header;
use dns::message::Label;
use dns::message::LabelSequence;
use dns::message::Message;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let received_message = Message::parse_from(&buf);

                let mut answer = Answer::new();
                answer.set_type(1);
                answer.set_class(1);
                answer.set_name(
                    received_message
                        .get_questions()
                        .iter()
                        .next()
                        .unwrap()
                        .get_name()
                        .clone(),
                );
                answer.set_ttl(60);
                answer.get_data().extend([0x8, 0x8, 0x8, 0x8]);
                let mut answers: Vec<Answer> = Vec::new();
                answers.push(answer);

                let mut header: Header = Header::default();
                header.set_id(received_message.get_header().get_id());
                header.set_qr(true);
                header.set_opcode(received_message.get_header().get_opcode());
                header.set_rd(received_message.get_header().get_rd());
                header.set_qd_count(received_message.get_header().get_qd_count());
                header.set_an_count(answers.len() as u16);

                let message =
                    Message::new(header, received_message.get_questions().clone(), answers);
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
