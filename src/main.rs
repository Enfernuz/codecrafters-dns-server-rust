use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
#[allow(unused_imports)]
use std::net::UdpSocket;
use std::os::unix::net::SocketAddr;

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
        println!("Resolver addr: {fwd_address}");
        let fwd_addr: SocketAddrV4 = fwd_address.parse().expect("KEK");
        fwd_socket
            .connect(fwd_addr)
            .expect("Failed to connect to FWD resolver");
        println!("FWD ADDR: {}", fwd_addr);
        Box::new(ForwardingDnsResolver {
            fwd_endpoint: fwd_socket,
        })
    } else {
        Box::new(DummyDnsResolver {})
    };

    let endpoint = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let server = DnsServer { endpoint, resolver };

    server.work();
}
