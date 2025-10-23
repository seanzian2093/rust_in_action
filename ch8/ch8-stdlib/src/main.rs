use std::io::prelude::*;
use std::io::stdout;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    // In many networking protocols, \r\n signifies a new line
    conn.write_all(b"\r\n")?;
    conn.write_all(b"Host: www.rustinaction.com")?;
    // In many networking protocols, \r\n\r\n signifies end of a request
    conn.write_all(b"\r\n\r\n")?;

    // streams bytes from conn to stdout
    std::io::copy(&mut conn, &mut stdout())?;

    Ok(())
}