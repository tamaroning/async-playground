use std::os::windows::prelude::OpenOptionsExt;
use tokio::{io, io::*};
use tokio::net::TcpListener;

#[tokio::main] 
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:10000").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await?;
    }
}