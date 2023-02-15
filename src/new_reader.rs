use {
    anyhow::Result,
    filedescriptor::FileDescriptor,
    socket2::{Domain, Protocol, Socket, Type},
    std::{
        net::{Ipv4Addr, SocketAddr},
        os::fd::AsRawFd,
        str::FromStr,
    },
};

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

fn mk_mcast_sock(mcast_grp: &str, mcast_port: i32) -> Result<FileDescriptor> {
    let mcast_host = "0.0.0.0";
    let mcast_sock = mk_sock()?;
    mcast_sock.join_multicast_v4(
        &Ipv4Addr::from_str(mcast_grp)?,
        &Ipv4Addr::from_str(mcast_host)?,
    )?;
    let bind_addr: SocketAddr = format!("{}:{}", mcast_host, mcast_port).as_str().parse()?;
    mcast_sock.bind(&bind_addr.into())?;
    Ok(FileDescriptor::dup(&mcast_sock)?)
}

fn open_mcast(uri: &str) -> Result<FileDescriptor> {
    // udp://@227.1.3.10:4310
    let uri: Vec<&str> = uri.split("udp://@").collect();
    let uri: Vec<&str> = uri[1].splitn(2, ":").collect();
    let mcast_grp = uri[0];
    let mcast_port = i32::from_str_radix(uri[1], 10)?;
    Ok(mk_mcast_sock(mcast_grp, mcast_port)?)
}

fn mk_udp_sock(udp_ip: &str, udp_port: i32) -> Result<FileDescriptor> {
    let udp_sock = mk_sock()?;
    let bind_addr: SocketAddr = format!("{}:{}", udp_ip, udp_port).as_str().parse()?;
    udp_sock.bind(&bind_addr.into())?;
    Ok(FileDescriptor::dup(&udp_sock)?)
}

fn open_udp(uri: &str) -> Result<FileDescriptor> {
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
        } else if uri.starts_with("udp://") {
            open_udp(uri)?;
        } else {
            let file = std::fs::File::open(uri)?;
            FileDescriptor::dup(&file)?;
        }
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
    fn test_file_reader() {
        let file_path = "./README.md";
        reader(Some(file_path)).unwrap();
    }

    #[test]
    fn test_udp_sock() {
        let udp_path = "udp://127.0.0.1:9090";
        reader(Some(udp_path)).unwrap()
    }
}
