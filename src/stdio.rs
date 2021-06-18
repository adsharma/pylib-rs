use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::Stdin;

pub trait FileReadString {
    fn read_string(&mut self) -> std::io::Result<String>;
}

impl FileReadString for File {
    fn read_string(&mut self) -> std::io::Result<String> {
        let mut contents = String::new();
        self.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub trait FileWriteString {
    fn write_string(&mut self, data: &str) -> std::io::Result<usize>;
}

impl FileWriteString for File {
    fn write_string(&mut self, data: &str) -> std::io::Result<usize> {
        let num = self.write(data.as_bytes())?;
        Ok(num)
    }
}

pub trait FileReadBytes {
    fn read_bytes(&mut self, n: u64) -> std::io::Result<Vec<u8>>;
}

impl FileReadBytes for File {
    fn read_bytes(&mut self, n: u64) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; n.try_into().unwrap()];
        let mut handle = self.take(n);
        handle.read_exact(&mut buf)?;
        Ok(buf)
    }
}

pub trait FileWriteBytes {
    fn write_bytes(&mut self, buf: &[u8]) -> std::io::Result<()>;
}

impl FileWriteBytes for File {
    fn write_bytes(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.write_all(&buf)?;
        Ok(())
    }
}

pub trait StdinReadLine {
    fn read_string(&mut self) -> std::io::Result<String>;
}

impl StdinReadLine for Stdin {
    fn read_string(&mut self) -> std::io::Result<String> {
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line)?;
        Ok(line)
    }
}
