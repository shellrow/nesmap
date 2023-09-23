use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use cross_socket::packet::ethernet::EtherType;
use cross_socket::packet::tcp::{TcpPacketBuilder, TcpFlag, TcpOption};
use cross_socket::socket::{Socket, DataLinkSocket, SocketOption, IpVersion, SocketType};
use cross_socket::packet::ip::IpNextLevelProtocol;
use cross_socket::datalink::interface::Interface;

// Send TCP SYN packets to 1.1.1.1:80 and check reply
// This example is for Unix(Linux, macOS ...) only.
// For Windows, use examples/datalink_socket/tcp_ping.rs instead.
// (Due to Winsock2 limitation.)
fn main() {
    let interface: Interface = cross_socket::datalink::interface::get_default_interface().unwrap();
    let src_ip: IpAddr = IpAddr::V4(interface.ipv4[0].addr);
    let dst_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let dst_socket_addr: SocketAddr = SocketAddr::new(dst_ip, 80);
    let socket_option = SocketOption {
        ip_version: IpVersion::V4,
        socket_type: SocketType::Raw,
        protocol: Some(IpNextLevelProtocol::Tcp),
        timeout: None,
        ttl: None,
        non_blocking: false,
    };
    // Sender socket
    let socket: Socket = Socket::new(socket_option).unwrap();

    // Receiver socket
    // RAW SOCKET recvfrom not working for TCP. So we use DataLinkSocket instead.
    let mut listener_socket: DataLinkSocket = DataLinkSocket::new(interface, false).unwrap();

    // Packet builder for TCP SYN
    let mut tcp_packet_builder = TcpPacketBuilder::new(SocketAddr::new(src_ip, 53443), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 80));
    tcp_packet_builder.flags = vec![TcpFlag::Syn];
    tcp_packet_builder.options = vec![TcpOption::Mss, TcpOption::SackParmitted, TcpOption::Nop, TcpOption::Nop, TcpOption::Wscale];

    // Send TCP SYN packet to 1.1.1.1
    match socket.send_to(&tcp_packet_builder.build(), dst_socket_addr) {
        Ok(packet_len) => {
            println!("Sent {} bytes", packet_len);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Receive packets
    println!("Waiting for TCP SYN+ACK... ");
    loop {
        match listener_socket.receive() {
            Ok(packet) => {
                let ethernet_packet = cross_socket::packet::ethernet::EthernetPacket::from_bytes(&packet);
                if ethernet_packet.ethertype != EtherType::Ipv4 {
                    continue;
                }
                let ip_packet = cross_socket::packet::ipv4::Ipv4Packet::from_bytes(&ethernet_packet.payload);
                if ip_packet.next_level_protocol != IpNextLevelProtocol::Tcp || ip_packet.source != std::net::Ipv4Addr::new(1, 1, 1, 1) {
                    continue;
                }
                println!("Received {} bytes from {}", packet.len(), ip_packet.source);
                let tcp_packet = cross_socket::packet::tcp::TcpPacket::from_bytes(&ip_packet.payload);
                println!("Packet: {:?}", tcp_packet);
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
