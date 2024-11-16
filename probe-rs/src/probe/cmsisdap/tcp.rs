use std::{
    cell::RefCell,
    io,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

// use tracing;

const TIMEOUT: Duration = Duration::from_millis(10000);

/// Auto-reconnecting TCP socket.
pub struct DurableStream {
    socket: RefCell<TcpStream>,
}

impl DurableStream {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let socket = TcpStream::connect(addr)?;
        return Ok(DurableStream {
            socket: RefCell::new(socket),
        });
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut socket = self.socket.borrow_mut();
        socket
            .set_read_timeout(Some(TIMEOUT))
            .expect("Non-zero read timeout");
        match socket.read(buf) {
            Ok(count) => Ok(count),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => todo!(),
                io::ErrorKind::PermissionDenied => todo!(),
                io::ErrorKind::ConnectionRefused => todo!(),
                io::ErrorKind::ConnectionReset => todo!(),
                io::ErrorKind::ConnectionAborted => todo!(),
                io::ErrorKind::NotConnected => todo!(),
                io::ErrorKind::AddrInUse => todo!(),
                io::ErrorKind::AddrNotAvailable => todo!(),
                io::ErrorKind::BrokenPipe => todo!(),
                io::ErrorKind::AlreadyExists => todo!(),
                io::ErrorKind::WouldBlock => todo!(),
                io::ErrorKind::InvalidInput => todo!(),
                io::ErrorKind::InvalidData => todo!(),
                io::ErrorKind::TimedOut => todo!(),
                io::ErrorKind::WriteZero => todo!(),
                io::ErrorKind::Interrupted => todo!(),
                io::ErrorKind::Unsupported => todo!(),
                io::ErrorKind::UnexpectedEof => todo!(),
                io::ErrorKind::OutOfMemory => todo!(),
                io::ErrorKind::Other => todo!(),
                _ => todo!(),
            },
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, io::Error> {
        let mut socket = self.socket.borrow_mut();
        // socket
        //     .set_write_timeout(Some(TIMEOUT))
        //     .expect("Non-zero write timeout");
        match socket.write(buf) {
            Ok(count) => Ok(count),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => todo!(),
                io::ErrorKind::PermissionDenied => todo!(),
                io::ErrorKind::ConnectionRefused => todo!(),
                io::ErrorKind::ConnectionReset => todo!(),
                io::ErrorKind::ConnectionAborted => todo!(),
                io::ErrorKind::NotConnected => todo!(),
                io::ErrorKind::AddrInUse => todo!(),
                io::ErrorKind::AddrNotAvailable => todo!(),
                io::ErrorKind::BrokenPipe => todo!(),
                io::ErrorKind::AlreadyExists => todo!(),
                io::ErrorKind::WouldBlock => todo!(),
                io::ErrorKind::InvalidInput => todo!(),
                io::ErrorKind::InvalidData => todo!(),
                io::ErrorKind::TimedOut => todo!(),
                io::ErrorKind::WriteZero => todo!(),
                io::ErrorKind::Interrupted => todo!(),
                io::ErrorKind::Unsupported => todo!(),
                io::ErrorKind::UnexpectedEof => todo!(),
                io::ErrorKind::OutOfMemory => todo!(),
                io::ErrorKind::Other => todo!(),
                _ => todo!(),
            },
        }
    }

    pub fn drain(&self, buffer: &mut [u8]) {
        let mut socket = self.socket.borrow_mut();
        socket
            .set_read_timeout(Some(Duration::from_millis(1)))
            .expect("Non-zero read timeout");
        loop {
            match socket.read(buffer) {
                Ok(n) if n != 0 => continue,
                _ => break,
            }
        }
    }
}
