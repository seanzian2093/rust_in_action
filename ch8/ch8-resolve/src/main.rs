use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{Command, Arg};
use rand;
use trust_dns_proto::op::{Message, MessageType, OpCode, Query};
use trust_dns_proto::rr::domain::Name;
use trust_dns_proto::rr::record_type::RecordType;
use trust_dns_proto::serialize::binary::*;

fn main() {
    let app = Command::new("ch8-resolve")
        .about("A simple to use DNS resolver")
        .arg(
            Arg::new("dns-server")
            .long("dns-server")
            .short('s')
            .default_value("1.1.1.1")
        )
        .arg(
            Arg::new("domain-name")
                .required(true)
        )
        .get_matches();

    let domain_name_raw = app.get_one::<String>("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap();

    let dns_server_raw = app.get_one::<String>("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw).parse().expect("invalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);

    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).expect("error emitting bytes");

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("failed to bind to address");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).expect("failed to set read timeout");
    localhost.set_nonblocking(false).expect("failed to set nonblocking");

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");
    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("error receiving response");

    let dns_message = Message::from_vec(&response_as_bytes[.._amt]).expect("error parsing response message");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            if let Some(ipv4) = answer.data() {
                println!("{}", ipv4);
            }
        }
    }
}
