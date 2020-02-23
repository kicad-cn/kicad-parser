use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::DerefMut;
use std::str;
use std::str::pattern::Pattern;
use std::str::Utf8Error;
use std::string::String;
use std::sync::{Arc, Mutex};

pub trait SCHRead {
    fn next_line(&mut self) -> io::Result<String>;
    fn as_str(&mut self) -> Result<&str, Utf8Error>;
}
pub struct SCHBufReader<T>
where
    T: Read,
{
    pub reader: Arc<Mutex<BufReader<T>>>,
    buf: Vec<u8>,
}

impl<T: Read> SCHBufReader<T> {
    pub fn New(readable: T) -> SCHBufReader<T> {
        return SCHBufReader::<T> {
            reader: Arc::new(Mutex::new(BufReader::new(readable))),
            buf: vec![],
        };
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        match self.reader.lock() {
            Ok(mut guard) => guard.deref_mut().read_line(buf),
            Err(_) => Err(Error::new(ErrorKind::Other, "mutex error")),
        }
    }
}
impl<T> SCHRead for SCHBufReader<T>
where
    T: Read,
{
    fn next_line(&mut self) -> io::Result<String> {
        let mut output = String::from("");
        while let Ok(lineSize) = self.read_line(&mut output) {
            if lineSize > 0 && !output.is_prefix_of("#") {
                return Ok(output);
            }
            if lineSize == 0 {
                return Err(Error::new(ErrorKind::Other, "EOF"));
            }
        }
        return Err(Error::new(ErrorKind::Other, "just error"));
    }
    fn as_str(&mut self) -> Result<&str, Utf8Error> {
        match self.reader.lock() {
            Ok(mut guard) => guard.deref_mut().read_to_end(&mut self.buf),
            Err(_) => Err(Error::new(ErrorKind::Other, "mutex error")),
        };
        return str::from_utf8(self.buf.as_slice());
    }
}
