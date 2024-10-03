use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, Error};
use std::mem::MaybeUninit;

fn main() -> io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    let mut buf = [MaybeUninit::uninit(); 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, src_addr)) => {
                let buf_ptr = buf.as_ptr() as *const u8;
                let packet_slice = unsafe { std::slice::from_raw_parts(buf_ptr, n) };

                if let Some(ipv4_packet) = Ipv4Packet::new(packet_slice) {
                    let ipv4_payload = ipv4_packet.payload();

                    if let Some(icmp_packet) = IcmpPacket::new(ipv4_payload) {
                        match icmp_packet.get_icmp_type() {
                            IcmpTypes::EchoRequest => {
                                if let Some(echo_request) =
                                    echo_request::EchoRequestPacket::new(icmp_packet.packet())
                                {
                                    println!("[Recv] Echo Request");
                                }
                            }
                            IcmpTypes::EchoReply => {
                                if let Some(echo_reply) =
                                    echo_reply::EchoReplyPacket::new(icmp_packet.packet())
                                {
                                    println!("[Recv] Echo Reply");
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => {
                return Err(Error::new(io::ErrorKind::Other, format!("{}", e)));
            }
        }
    }
}
