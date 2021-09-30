use std::net::{IpAddr, SocketAddr};
use crate::packet_parsing::types::{HandshakeData, MacAddress};


pub fn gen_pseudomac(addr: &SocketAddr) -> MacAddress {
    match addr.ip() {
        IpAddr::V4(ip) => {
            let octets = ip.octets();
            MacAddress(octets[0], octets[1], octets[2], octets[3], 0, 0)
        },
        IpAddr::V6(ip) => {
            let octets = ip.octets();
            MacAddress(octets[0], octets[1], octets[2],
                       octets[3], octets[4], octets[5])
        },
    }

}

pub fn ensure_pseudomac(data: &mut HandshakeData, addr: &SocketAddr) {
    if data.mac_address == MacAddress(0, 0, 0, 0, 0, 0) {
        data.mac_address = gen_pseudomac(addr);
    }
}