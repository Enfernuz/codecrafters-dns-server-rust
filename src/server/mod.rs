use std::{
    net::{SocketAddrV4, UdpSocket},
    os::unix::net::SocketAddr,
    rc::Rc,
};

mod dns;

use dns::message::{Answer, Header, Message, OpCode, Question, RCode};

pub struct DnsServer {
    pub endpoint: UdpSocket,
    pub resolver: Box<dyn Resolve>,
}

impl DnsServer {
    pub fn work(&self) {
        let mut buf = [0; 512];
        loop {
            match self.endpoint.recv_from(&mut buf) {
                Ok((size, source)) => {
                    println!("Received {} bytes from client at {}", size, source);
                    let request = Message::parse_from(&buf);
                    println!("Received DNS message:\n{}", &request);

                    let answers = self
                        .resolver
                        .resolve(&request.get_header(), request.get_questions());

                    let mut header: Header = Header::default();
                    header.set_id(request.get_header().get_id());
                    header.set_qr(true);
                    header.set_opcode(&request.get_header().get_opcode());
                    header.set_rd(request.get_header().get_rd());
                    header.set_rcode(&Rc::new(match request.get_header().get_opcode().as_ref() {
                        OpCode::Query => RCode::NoError,
                        _ => RCode::NotImplemented,
                    }));
                    header.set_qd_count(request.get_header().get_qd_count());
                    header.set_an_count(answers.len() as u16);

                    let response =
                        Message::new(&header.into(), request.get_questions(), &answers.into());
                    println!("Response:\n{}", &response);
                    let encoded_response = response.encode();
                    self.endpoint
                        .send_to(&encoded_response, source)
                        .expect("Failed to send response");
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    break;
                }
            }
        }
    }
}

pub struct DummyDnsResolver {}

pub struct ForwardingDnsResolver {
    pub fwd_endpoint: UdpSocket,
}

pub trait Resolve {
    fn resolve(&self, header: &Header, questions: &Rc<[Question]>) -> Rc<[Answer]>;
}

impl Resolve for DummyDnsResolver {
    fn resolve(&self, _header: &Header, questions: &Rc<[Question]>) -> Rc<[Answer]> {
        let mut answers: Vec<Answer> = Vec::new();
        for question in questions.as_ref() {
            answers.push(Answer::new(
                /* name= */ &question.get_name(),
                /* type= */ 1,
                /* class= */ 1,
                /* ttl= */ 60,
                /* data= */ &Vec::from_iter([0x8, 0x8, 0x8, 0x8]).into(),
            ));
        }
        answers.into()
    }
}

impl Resolve for ForwardingDnsResolver {
    fn resolve(&self, header: &Header, questions: &Rc<[Question]>) -> Rc<[Answer]> {
        let mut fwd_header_stub = Header::default();
        fwd_header_stub
            .set_id(header.get_id())
            .set_qr(false)
            .set_opcode(header.get_opcode())
            .set_rd(header.get_rd())
            .set_qd_count(1);
        let fwd_header = Rc::new(fwd_header_stub);

        let mut answers: Vec<Answer> = Vec::new();
        for question in questions.as_ref() {
            let fwd_request = Message::new(&fwd_header, &[question.clone()].into(), &[].into());
            println!("[FORWARD] Request:\n{}", &fwd_request);
            self.fwd_endpoint
                .send(&fwd_request.encode())
                .expect("Failed to send message to the DNS resolver.");
            println!("Sent DNS query to the resolver");
            let mut buf = [0; 512];
            match self.fwd_endpoint.recv_from(&mut buf) {
                Ok((sz, src)) => {
                    println!("Received {} bytes from the resolver at {}.", sz, &src);
                    let fwd_response = Message::parse_from(&buf);
                    println!("Received response from the resolver: {}", &fwd_response);
                    fwd_response.get_answers().iter().for_each(|answer| {
                        println!("Pushing fwd answer:\n{}", answer.clone());
                        answers.push(answer.clone());
                    });
                }
                Err(err) => {
                    println!("Error receiving from the resolver: {}", &err);
                }
            }
        }

        answers.into()
    }
}
