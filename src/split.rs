use std::io::{self, Read, Write};

use futures::Async;
use futures::sync::BiLock;

use {AsyncRead, AsyncWrite};

/// The readable half of an object returned from `AsyncRead::split`.
pub struct ReadHalf<T> {
    handle: BiLock<T>,
}

/// The writable half of an object returned from `AsyncRead::split`.
pub struct WriteHalf<T> {
    handle: BiLock<T>,
}

pub fn split<T: AsyncRead + AsyncWrite>(t: T) -> (ReadHalf<T>, WriteHalf<T>) {
    let (a, b) = BiLock::new(t);
    (ReadHalf { handle: a }, WriteHalf { handle: b })
}

fn would_block() -> io::Error {
    io::Error::new(io::ErrorKind::WouldBlock, "would block")
}

impl<T: AsyncRead> Read for ReadHalf<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.read(buf),
            Async::NotReady => Err(would_block()),
        }
    }
}

impl<T: AsyncRead> AsyncRead for ReadHalf<T> {
    fn poll_read(&mut self) -> Async<()> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.poll_read(),
            Async::NotReady => Async::NotReady,
        }
    }
}

impl<T: AsyncWrite> Write for WriteHalf<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.write(buf),
            Async::NotReady => Err(would_block()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.flush(),
            Async::NotReady => Err(would_block()),
        }
    }
}

impl<T: AsyncWrite> AsyncWrite for WriteHalf<T> {
    fn poll_write(&mut self) -> Async<()> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.poll_write(),
            Async::NotReady => Async::NotReady,
        }
    }
}
