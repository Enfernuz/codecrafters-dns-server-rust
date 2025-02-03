#[allow(unused_imports)]
use std::net::UdpSocket;

mod dns;
use dns::message::Answer;
use dns::message::Header;
use dns::message::Label;
use dns::message::Message;
use dns::message::Question;

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
                question.set_type(1);
                question.set_class(1);
                question
                    .get_name()
                    .get_labels()
                    .extend([Label::new("codecrafters"), Label::new("io")]);
                let mut questions: Vec<Question> = Vec::new();
                questions.push(question);

                let mut answer = Answer::new();
                answer.set_type(1);
                answer.set_class(1);
                answer
                    .get_name()
                    .get_labels()
                    .extend([Label::new("codecrafters"), Label::new("io")]);
                answer.set_ttl(60);
                answer.get_data().extend([0x8, 0x8, 0x8, 0x8]);
                let mut answers: Vec<Answer> = Vec::new();
                answers.push(answer);

                let mut header: Header = Header::default();
                header.set_id(1234);
                header.set_qr(true);
                header.set_qd_count(questions.len() as u16);
                header.set_an_count(answers.len() as u16);

                let message = Message::new(header, questions, answers);
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
