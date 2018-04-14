mod byte_packet_buffer;
mod result_code;
mod dns_header;
mod query_type;
mod dns_question;
mod dns_record;
mod dns_packet;

use std::net::UdpSocket;
use query_type::QueryType;
use dns_packet::DnsPacket;
use dns_question::DnsQuestion;
use byte_packet_buffer::BytePacketBuffer;

fn main() {
    // let qname = "google.com";
    let qname = "www.yahoo.com";
    // let qtype = QueryType::A;
    let qtype = QueryType::MX;

    let server = ("8.8.8.8", 53);
    let socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();

    socket
        .send_to(&req_buffer.buf[0..req_buffer.pos], server)
        .unwrap();

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    let res_packet = DnsPacket::from_buffer(&mut res_buffer).unwrap();

    println!("{:?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:?}", q);
    }

    for rec in res_packet.answers {
        println!("{:?}", rec);
    }

    for rec in res_packet.authorities {
        println!("{:?}", rec);
    }

    for rec in res_packet.resources {
        println!("{:?}", rec);
    }
}
