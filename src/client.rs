use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt};
pub struct Client {
    tcp: TcpStream,
    pub username: String,
    pub password: String
}

impl Client {
    async fn connect(host: String, username: String, password: String) -> io::Result<Self> {
        let mut tcp = TcpStream::connect(host).await?;
        let mut b = BytesMut::with_capacity(1024);
        tcp.read_buf(&mut b).await?;
        Ok(Self {tcp, username, password})

    }
}
