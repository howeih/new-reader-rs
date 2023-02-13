use anyhow::Result;
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    any::TypeId,
    io::{BufRead, Read},
    net::TcpStream,
};
trait Process<T> {
    fn reader(&self);
}

fn double_rcvbuf(sock: &Socket) -> Result<()> {
    let rcvbuf_size = sock.recv_buffer_size()?;
    sock.set_recv_buffer_size(2 * rcvbuf_size)?;
    Ok(())
}

fn mk_sock() -> Result<Socket> {
    let sock = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    double_rcvbuf(&sock)?;
    sock.set_reuse_port(true)?;
    Ok(sock)
}

fn mk_mcast_sock(mcast_grp: &str, mcast_post: i32) -> Result<()> {
    let mcast_host = "0.0.0.0";
    let mcast_sock = mk_sock()?;

    Ok(())
}

fn open_mcast(uri: &str) -> Result<()> {
    let s: Vec<&str> = uri.split("udp://@").collect();
    let s2: Vec<&str> = s[1].splitn(2, ":").collect();
    let mcast_grp = s2[0];
    let mcast_port = i32::from_str_radix(s2[1], 10)?;

    println!("{:} {:}", s2[0], s2[1]);
    Ok(())
}

fn open_udp(uri: &str) {}

pub fn reader<T>(uri: Option<T>)
where
    T: AsRef<str>,
{
    let a = if let Some(uri) = uri {
        let uri = uri.as_ref();
        if uri.starts_with("udp://@") {
            open_mcast(uri);
        }
        if uri.starts_with("udp://") {}
        if uri.starts_with("http") {}
    } else {
        let stdin = std::io::stdin();
    };
}

#[cfg(test)]
mod test {
    use crate::new_reader::reader;
    #[test]
    fn test_reader() {
        let s = "udp://@227.1.3.10:4310".to_string();
        reader(Some(s));
    }
}
