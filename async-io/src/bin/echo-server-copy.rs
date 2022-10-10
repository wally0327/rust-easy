use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:7142").await?;
    let (mut rd, mut wr) = io::split(socket);

    let t = tokio::spawn(async move {
        let _r1 = wr.write_all(b"hello\r\n").await;
        let _r2 = wr.write_all(b"world\r\n").await;

        Ok::<_, io::Error>(());
        // todo: 怎么在 task 中处理 Result
    });
    t.await?;

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        println!("Got {:?}", &buf[..n]);
    }
    Ok(())
}
