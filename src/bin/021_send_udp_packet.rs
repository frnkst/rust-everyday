use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:34254")?;
        let address = "8.8.8.8:80";
        let message ="test message".as_bytes();
        socket.send_to(message, address)?;
    }
    Ok(())
}
