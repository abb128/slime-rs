use std::{collections::HashMap, fmt::Debug, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}};

use crate::{connection::{client::Client, listener::{Listener, RemoteMap}}, packet_parsing::{server, client, types::{HandshakeData, MacAddress}}};
use std::time::SystemTime;

use super::enums::{BackendDataMutRef, BackendDataRef, BackendRemoteData, BackendType};

pub struct UdpServer {
    socket: UdpSocket,
    addr_to_mac: HashMap<SocketAddr, MacAddress>,
    buf: [u8; 256],
    local_addr: SocketAddr,
}

fn ensure_pseudomac(data: &mut HandshakeData, addr: &SocketAddr) {
    if data.mac_address == MacAddress(0, 0, 0, 0, 0, 0) {
        match addr.ip() {
            IpAddr::V4(ip) => {
                let octets = ip.octets();
                data.mac_address.0 = octets[0];
                data.mac_address.1 = octets[1];
                data.mac_address.2 = octets[2];
                data.mac_address.3 = octets[3];
            },
            IpAddr::V6(ip) => {
                let octets = ip.octets();
                data.mac_address.0 = octets[0];
                data.mac_address.1 = octets[1];
                data.mac_address.2 = octets[2];
                data.mac_address.3 = octets[3];
                data.mac_address.4 = octets[4];
                data.mac_address.5 = octets[5];
            },
        }
        
    }
}

impl UdpServer {
    fn get_udp_client_mut<'a>(&self, client: &'a mut Client) -> Option<&'a mut UdpClient> {
        if let Some(BackendDataMutRef::Udp(udp))
        = client.find_client_type_mut(&BackendType::Udp(self.local_addr))
        {
            return Some(udp);
        }
        
        return None;
    }

    fn get_udp_client<'a>(&self, client: &'a Client) -> Option<&'a UdpClient> {
        if let Some(BackendDataRef::Udp(udp))
        = client.find_client_type(&BackendType::Udp(self.local_addr))
        {
            return Some(udp);
        }
        
        return None;
    }
    
    pub fn new(port: u16) -> Result<UdpServer, std::io::Error> {
        let mut srv = UdpServer {
            socket: UdpSocket::bind(("0.0.0.0", port))?,
            addr_to_mac: Default::default(),
            buf: [0u8; 256],
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port),
        };
        
        if let Ok(addr) = srv.socket.local_addr() {
            srv.local_addr = addr;
        }

        srv.socket.set_nonblocking(true)?;

        Ok(srv)
    }

    pub fn receive_packet(&mut self, size: usize, addr: SocketAddr, client_map: &mut RemoteMap) -> Option<()> {
        let dec = server::parse_slice(&self.buf[0..size])?;

        if let server::PacketType::Handshake(_, mut dat) = dec {
            // If out-of-date handshake is used, skip
            if size == 12 {
                return None;
            }

            // Create new client, insert to addr_to_mac
            
            // Ensure mac address is not blank (iOS owoTrack, old Android app)
            ensure_pseudomac(&mut dat, &addr);

            // Insert socketaddr to mac mapping
            self.addr_to_mac.insert(addr, dat.mac_address.clone());

            // Create client
            let mac_key = dat.mac_address.clone();
            let client = Client::new(&dat);
            
            // Insert to hashmap
            let c = client_map.entry(mac_key).or_insert(client);

            // Insert UdpClient into c
            let udp_client = UdpClient {
                srv_addr: self.local_addr,
                last_addr: addr,
                last_activity: SystemTime::now()
            };
            let insert_result = c.insert_client_type(BackendType::Udp(self.local_addr), Box::new(udp_client));
            if let Err(msg) = insert_result {
                println!("Failed to insert client: {}", msg);
            }

            // now notify client so it can respond
            c.handle_handshake(dat);
        } else if let Some(mac) = self.addr_to_mac.get(&addr) {
            if let Some(client) = client_map.get_mut(mac) {
                client.receive_packet(dec);
                
                if let Some(udp) = self.get_udp_client_mut(client) {
                    udp.last_addr = addr;
                    udp.last_activity = SystemTime::now();
                }
            }
        }

        None
    }
}

impl Listener for UdpServer {
    fn receive(&mut self, client_map: &mut RemoteMap) {
        loop {
            let r = self.socket.recv_from(&mut self.buf);
            match r {
                Ok((size, addr)) => {
                    self.receive_packet(size, addr, client_map);
                }
                Err(_) => break
            }
        }
    }

    fn flush(&mut self, client_map: &mut RemoteMap) {
        for (_, client) in client_map.iter_mut() {
            let outgoing = client.get_outgoing_packets();
            if outgoing.len() == 0 { continue; }
            
            if let Some(udp) = self.get_udp_client(client) {
                if udp.srv_addr != self.local_addr {
                    println!("MISMATCH!!");
                    continue;
                }

                for packet in outgoing {
                    // TODO: would be more efficient to only encode once, than per every client
                    let encoded = client::to_bytes(packet);
                    if let Some(bytes) = encoded {
                        let result = self.socket.send_to(bytes.as_slice(), udp.last_addr);
                        if let Err(v) = result { // TODO: prettier error handling
                                                       // TODO: dont remove this packet, retry next time around
                                                       // TODO: kill connection if never gets through? for example it might have disconnected from wifi, related to TODO 1
                            println!("Failed to send packet: {}", v);
                        }else if let Ok(v) = result {
                            println!("Send {} packets to {}! (from {})", v, udp.last_addr, self.local_addr);
                        }
                    }
                }
            }
        }
    }
}


#[derive(Debug)]
pub struct UdpClient {
    srv_addr: SocketAddr,
    last_addr: SocketAddr,
    last_activity: SystemTime // TODO: use this to determine if the client is still active
                              // TODO 1: ClientServer trait have method is_alive() -> bool, determine based on last activity in case of UDP
                              // TODO: in case of Bluetooth or TCP, a more reliable method can be used
}

impl BackendRemoteData for UdpClient {
    fn get_data(&self) -> BackendDataRef<'_> {
        BackendDataRef::Udp(self)
    }

    fn get_data_mut(&mut self) -> BackendDataMutRef<'_> {
        BackendDataMutRef::Udp(self)
    }
}