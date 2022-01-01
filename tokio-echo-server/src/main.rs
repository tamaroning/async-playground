use tokio::net::TcpListener;
use tokio::{io, io::*};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Create TCP listener
    let listen_addr = "127.0.0.1:10000";
    // Listen
    let listener = TcpListener::bind(listen_addr).await.unwrap();
    println!("TCP listener: {}", listen_addr);

    loop {
        // Accept a TCP connection
        let (mut socket, addr) = listener.accept().await?;
        println!("accept: {}", addr);

        // Create an async task
        tokio::spawn(async move {
            let (r, w) = socket.split();
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);
            let mut line = String::new();
            loop {
                line.clear();
                // Read a line
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("read: {}, {}", addr, line);
                        // Echo
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
