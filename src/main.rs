#[allow(unused_imports)]
use std::net::SocketAddrV4;
use std::net::UdpSocket;

mod cli;
use clap::Parser;
use cli::CliArgs;

mod server;

use server::DnsServer;
use server::DummyDnsResolver;
use server::ForwardingDnsResolver;
use server::Resolve;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let cli: CliArgs = CliArgs::parse();

    let resolver: Box<dyn Resolve> = if let Some(fwd_address) = cli.resolver {
        let fwd_socket =
            UdpSocket::bind("0.0.0.0:2060").expect("Failed to bind to DNS resolver address");
        let fwd_addr: SocketAddrV4 = fwd_address.parse().expect("Failed to parse IPv4 address.");
        println!("DNS resolver type: Forward (will forward DNS requests to {fwd_address}).");
        fwd_socket
            .connect(fwd_addr)
            .expect("Failed to connect to forward DNS resolver");
        Box::new(ForwardingDnsResolver {
            fwd_endpoint: fwd_socket,
        })
    } else {
        println!("DNS resolver type: Dummy (will respond with fake data).");
        Box::new(DummyDnsResolver {})
    };

    let endpoint = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let server = DnsServer { endpoint, resolver };

    server.work();
}
