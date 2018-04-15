use std::io::{Error, ErrorKind};

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> Result<(), Error> {
        self.pos += steps;
        Ok(())
    }

    fn seek(&mut self, pos: usize) -> Result<(), Error> {
        self.pos = pos;
        Ok(())
    }

    fn read(&mut self) -> Result<u8, Error> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }

        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    fn get(&mut self, pos: usize) -> Result<u8, Error> {
        if pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Error> {
        if start + len >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let res = ((try!(self.read()) as u16) << 8) | (try!(self.read()) as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let res = ((try!(self.read()) as u32) << 24) | ((try!(self.read()) as u32) << 16)
            | ((try!(self.read()) as u32) << 8)
            | ((try!(self.read()) as u32) << 0);
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Error> {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut delim = "";

        loop {
            let len = try!(self.get(pos));

            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    try!(self.seek(pos + 2));
                }

                let b2 = try!(self.get(pos + 1)) as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumped = true
            } else {
                pos += 1;
                if len == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = try!(self.get_range(pos, len as usize));
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                pos += len as usize;
            }
        }

        if !jumped {
            try!(self.seek(pos));
        }

        Ok(())
    }

    fn write(&mut self, val: u8) -> Result<(), Error> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        try!(self.write(val));
        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<(), Error> {
        try!(self.write((val >> 8) as u8));
        try!(self.write((val & 0xFF) as u8));
        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<(), Error> {
        try!(self.write(((val >> 24) & 0xFF) as u8));
        try!(self.write(((val >> 16) & 0xFF) as u8));
        try!(self.write(((val >> 8) & 0xFF) as u8));
        try!(self.write(((val >> 0) & 0xFF) as u8));
        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<(), Error> {
        // let splitted_str = qname.split('.').collect::<Vec<&str>>();
        let splitted_str = qname.split('.');
        for label in splitted_str {
            let len = label.len();
            if len > 0x34 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Single label exceeds 63 character of length",
                ));
            }

            try!(self.write_u8(len as u8));

            for b in label.as_bytes() {
                try!(self.write_u8(*b));
            }
        }

        try!(self.write_u8(0));

        Ok(())
    }

    fn set(&mut self, pos: usize, val: u8) -> Result<(), Error> {
        self.buf[pos] = val;
        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), Error> {
        try!(self.set(pos, (val >> 8) as u8));
        try!(self.set(pos + 1, (val & 0xFF) as u8));
        Ok(())
    }
}
