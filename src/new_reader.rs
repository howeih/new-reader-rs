use anyhow::Result;
use bytes::Bytes;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{
    any::TypeId,
    net::{Ipv4Addr, SocketAddr},
    os::fd::AsRawFd,
    str::FromStr,
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

fn mk_mcast_sock(mcast_grp: &str, mcast_port: i32) -> Result<i32> {
    let mcast_host = "0.0.0.0";
    let mcast_sock = mk_sock()?;
    mcast_sock.join_multicast_v4(
        &Ipv4Addr::from_str(mcast_grp)?,
        &Ipv4Addr::from_str(mcast_host)?,
    )?;
    let bind_addr: SocketAddr = format!("{}:{}", mcast_host, mcast_port).as_str().parse()?;
    mcast_sock.bind(&bind_addr.into())?;
    Ok(mcast_sock.as_raw_fd())
}

fn open_mcast(uri: &str) -> Result<i32> {
    // udp://@227.1.3.10:4310
    let uri: Vec<&str> = uri.split("udp://@").collect();
    let uri: Vec<&str> = uri[1].splitn(2, ":").collect();
    let mcast_grp = uri[0];
    let mcast_port = i32::from_str_radix(uri[1], 10)?;
    Ok(mk_mcast_sock(mcast_grp, mcast_port)?)
}

fn mk_udp_sock(udp_ip: &str, udp_port: i32) -> Result<i32> {
    let udp_sock = mk_sock()?;
    let bind_addr: SocketAddr = format!("{}:{}", udp_ip, udp_port).as_str().parse()?;
    udp_sock.bind(&bind_addr.into())?;
    Ok(udp_sock.as_raw_fd())
}

fn open_udp(uri: &str) -> Result<i32> {
    // udp://1.2.3.4:5555
    let uri: Vec<&str> = uri.split("udp://").collect();
    let uri: Vec<&str> = uri[1].splitn(2, ":").collect();
    let udp_ip = uri[0];
    let udp_port = i32::from_str_radix(uri[1], 10)?;
    Ok(mk_udp_sock(udp_ip, udp_port)?)
}

pub fn reader<T>(uri: Option<T>) -> Result<()>
where
    T: AsRef<str>,
{
    if let Some(uri) = uri {
        let uri = uri.as_ref();
        if uri.starts_with("udp://@") {
            open_mcast(uri)?;
        }
        if uri.starts_with("udp://") {
            open_udp(uri)?;
        }
        if uri.starts_with("http") {}
    } else {
        let stdin = std::io::stdin();
        stdin.as_raw_fd();
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::new_reader::reader;
    #[test]
    fn test_reader() {
        let s = "udp://@227.1.3.10:4310".to_string();
        reader(Some(s)).unwrap();
    }
}
