#[allow(unused_imports)]
use std::net::UdpSocket;
use std::rc::Rc;

mod dns;
use dns::message::Answer;
use dns::message::Header;
use dns::message::Message;
use dns::message::OpCode;
use dns::message::RCode;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let mut forward_socket: Option<UdpSocket>;
    if let Some(addr) = get_resolver_address() {
        let sock =
            UdpSocket::bind("127.0.0.1:2060").expect("Failed to bind to DNS resolver address");
        sock.connect(addr)
            .expect("Failed to connect to DNS resolver address");
        forward_socket = Some(sock);
    } else {
        forward_socket = None;
    }

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from client at {}", size, source);

                let received_message = Message::parse_from(&buf);
                println!("Request:\n{}", &received_message);

                let mut answers: Vec<Answer> = Vec::new();
                for question in received_message.get_questions() {
                    if let Some(fwd_sock) = forward_socket.as_mut() {
                        let mut header: Header = Header::default();
                        header.set_id(received_message.get_header().get_id());
                        header.set_qr(false);
                        header.set_opcode(&received_message.get_header().get_opcode());
                        header.set_rd(false);
                        header.set_qd_count(1);
                        let message = Message::new(
                            &Rc::new(header),
                            Vec::from_iter([question.clone()]),
                            Vec::new(),
                        );
                        fwd_sock
                            .send(message.encode().as_slice())
                            .expect("Failed to send message to the DNS resolver.");
                        println!("Sent DNS query to the resolver");
                        let mut fwd_buf = [0; 512];
                        match fwd_sock.recv_from(&mut fwd_buf) {
                            Ok((sz, src)) => {
                                println!("Received {} bytes from the resolver at {}.", sz, &src);
                                let recv = Message::parse_from(&fwd_buf);
                                recv.get_answers().iter().for_each(|a| {
                                    let mut ans = Answer::new();
                                    ans.set_name(a.get_name().clone());
                                    ans.set_type(a.get_type());
                                    ans.set_class(a.get_class());
                                    ans.set_ttl(a.get_ttl());
                                    ans.set_data(a.get_data().clone());
                                    answers.push(ans);
                                });
                            }
                            Err(err) => {
                                println!("Error receiving from the resolver: {}", &err);
                            }
                        }
                    } else {
                        let mut answer = Answer::new();
                        answer.set_type(1);
                        answer.set_class(1);
                        answer.set_name(question.get_name().clone());
                        answer.set_ttl(60);
                        answer.set_data(Vec::from_iter([0x8, 0x8, 0x8, 0x8]));
                        answers.push(answer);
                    }
                }

                let mut header: Header = Header::default();
                header.set_id(received_message.get_header().get_id());
                header.set_qr(true);
                header.set_opcode(&received_message.get_header().get_opcode());
                header.set_rd(received_message.get_header().get_rd());
                header.set_rcode(&Rc::new(
                    match received_message.get_header().get_opcode().as_ref() {
                        OpCode::Query => RCode::NoError,
                        _ => RCode::NotImplemented,
                    },
                ));
                header.set_qd_count(received_message.get_header().get_qd_count());
                header.set_an_count(answers.len() as u16);

                let message = Message::new(
                    &Rc::new(header),
                    received_message.get_questions().clone(),
                    answers,
                );
                println!("Response:\n{}", &message);
                let response = message.encode();

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

fn get_resolver_address() -> Option<String> {
    std::env::args().nth(2)
}
