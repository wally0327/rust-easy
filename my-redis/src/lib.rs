use bytes::{Buf, BytesMut};
use mini_redis::{Frame, Result};
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection reset by peer".into())
                };
            }
        }
    }

    pub async fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(self.buffer.to_vec().as_slice());

        match Frame::check(&mut buf[..]) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }

            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    //pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {}
}
