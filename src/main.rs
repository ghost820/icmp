use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, Error};
use std::mem::MaybeUninit;

fn main() -> io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    let mut buf = [MaybeUninit::uninit(); 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, src_addr)) => {
                println!("{}", n);
            }
            Err(e) => {
                return Err(Error::new(io::ErrorKind::Other, format!("{}", e)));
            }
        }
    }
}
