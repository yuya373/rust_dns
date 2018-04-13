mod byte_packet_buffer;
mod result_code;
mod dns_header;
mod query_type;
mod dns_question;
mod dns_record;
mod dns_packet;

use std::fs::File;
use std::io::prelude::*;
use byte_packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;

fn main() {
    let mut f = File::open("response_packet.txt").unwrap();
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf).unwrap();

    let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
    println!("{:?}", packet.header);

    for q in packet.questions {
        println!("{:?}", q);
    }

    for rec in packet.answers {
        println!("{:?}", rec);
    }

    for rec in packet.authorities {
        println!("{:?}", rec);
    }

    for rec in packet.resources {
        println!("{:?}", rec);
    }
}
