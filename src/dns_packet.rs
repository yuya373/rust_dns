use std::io::Error;
use dns_header::DnsHeader;
use dns_question::DnsQuestion;
use dns_record::DnsRecord;
use byte_packet_buffer::BytePacketBuffer;
use query_type::QueryType;

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket, Error> {
        let mut result = DnsPacket::new();
        try!(result.header.read(buffer));

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            try!(question.read(buffer));
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = try!(DnsRecord::read(buffer));
            result.answers.push(rec);
        }

        for _ in 0..result.header.authoritative_entries {
            let rec = try!(DnsRecord::read(buffer));
            result.authorities.push(rec);
        }

        for _ in 0..result.header.resource_entries {
            let rec = try!(DnsRecord::read(buffer));
            result.resources.push(rec);
        }

        Ok(result)
    }

    pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Error> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authoritative_entries = self.authorities.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;

        try!(self.header.write(buffer));

        for q in &self.questions {
            try!(q.write(buffer));
        }

        for rec in &self.answers {
            try!(rec.write(buffer));
        }

        for rec in &self.authorities {
            try!(rec.write(buffer));
        }

        for rec in &self.resources {
            try!(rec.write(buffer));
        }

        Ok(())
    }
}
