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

    fn pos(&self) -> usize {
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

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Error> {
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
}
