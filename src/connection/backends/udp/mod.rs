use std::{collections::HashMap, fmt::Debug, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}};

use crate::{connection::{remote_client::{Client, ClientsContainer, PacketBuffered, RemoteClientWrapper, Server}, listener::{Listener, RemoteMap}}, packet_parsing::{client::{self, ClientHandshake}, server, types::{HandshakeData, MacAddress}}};
use std::time::SystemTime;

use super::enums::{BackendDataMutRef, BackendDataRef, BackendRemoteData, BackendType};

pub struct UdpServer {
    socket: UdpSocket,
    addr_to_mac: HashMap<SocketAddr, MacAddress>,
    buf: [u8; 256],
    local_addr: SocketAddr,
}

fn gen_pseudomac(addr: &SocketAddr) -> MacAddress {
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

fn ensure_pseudomac(data: &mut HandshakeData, addr: &SocketAddr) {
    if data.mac_address == MacAddress(0, 0, 0, 0, 0, 0) {
        data.mac_address = gen_pseudomac(addr);
    }
}

impl UdpServer {
    fn insert_udp_wrapper<'a>(&self, addr: SocketAddr, client: &'a mut RemoteClientWrapper) -> Option<&'a mut UdpClient>{
        let udp_client = UdpClient {
            srv_addr: self.local_addr,
            last_addr: addr,
            last_activity: SystemTime::now()
        };

        let insert_result = client.insert_client_type(BackendType::Udp(self.local_addr), Box::new(udp_client));
        
        if let Ok(result) = insert_result {
            let data = result.get_data_mut();

            if let BackendDataMutRef::Udp(udp ) = data {
                return Some(udp);
            }else{
                panic!("Inserted a Udp backendtype, received a different one");
            }
        }
        
        return None;
    }

    fn handle_client2server_handshake(&mut self, mut dat: HandshakeData, size: usize, addr: SocketAddr, client_map: &mut RemoteMap) -> Option<()> {
        // If out-of-date handshake is used, skip
        if size == 12 {
            return None;
        }
    
        // Ensure mac address is not blank (iOS owoTrack, old Android app)
        ensure_pseudomac(&mut dat, &addr);

        // Insert socketaddr to mac mapping
        self.addr_to_mac.insert(addr, dat.mac_address.clone());

        // Create client
        let mac_key = dat.mac_address.clone();
        let client = Client::new(&dat);
            
        // Insert to hashmap
        let wrap: &mut RemoteClientWrapper = client_map.entry(mac_key).or_insert(RemoteClientWrapper::Client(client));

        // Insert UdpClient into wrapper
        if let Some(udp) = self.insert_udp_wrapper(addr, wrap){
        
        }else{
            println!("Failed to insert UdpClient..");
        }
            
        // Make sure we actually have a client..
        if let RemoteClientWrapper::Client(c) = wrap {
            // now notify client so it can respond
            c.handle_handshake(dat);
        }

        return None;
    }

    pub fn connect_to_server(&mut self, addr: SocketAddr, client_map: &mut RemoteMap){
        // Generate fake MAC address since we don't have it.
        // TODO: We should avoid doing this
        let mac = gen_pseudomac(&addr);

        // Insert socketaddr to mac mapping
        self.addr_to_mac.insert(addr, mac.clone());

        // Create server
        let mac_key = mac.clone();
        let server = Server::new(mac);
        
        // Insert to hashmap
        let wrap = client_map.entry(mac_key).or_insert(RemoteClientWrapper::Server(server));

        // Insert UdpClient into wrapper
        if let Some(udp) = self.insert_udp_wrapper(addr, wrap){
        
        }else{
            println!("Failed to insert UdpClient..");
        }

        // Make sure we actually have a server..
        if let RemoteClientWrapper::Server(s) = wrap {
            s.send_handshake_to_server();
        }
    }

    fn get_udp_client_mut<'a, T: ClientsContainer>(&self, rc: &'a mut T) -> Option<&'a mut UdpClient> {
        if let Some(BackendDataMutRef::Udp(udp))
        = rc.find_client_type_mut(&BackendType::Udp(self.local_addr))
        {
            return Some(udp);
        }
        
        return None;
    }

    fn get_udp_client<'a, T: ClientsContainer>(&self, rc: &'a T) -> Option<&'a UdpClient> {
        if let Some(BackendDataRef::Udp(udp))
        = rc.find_client_type(&BackendType::Udp(self.local_addr))
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
        if let Some(mac) = self.addr_to_mac.get(&addr) {
            if let Some(rc) = client_map.get_mut(mac) {
                match rc {
                    RemoteClientWrapper::Client(client) => {
                        let dec = server::parse_slice(&self.buf[0..size])?;
                        client.receive_packet(dec);
                        
                        if let Some(udp) = self.get_udp_client_mut(client) {
                            udp.last_addr = addr;
                            udp.last_activity = SystemTime::now();
                        }
                    },

                    RemoteClientWrapper::Server(server) => {
                        let dec = client::parse_slice(&self.buf[0..size])?;
                        server.receive_packet(dec);
                        
                        if let Some(udp) = self.get_udp_client_mut(server) {
                            udp.last_addr = addr;
                            udp.last_activity = SystemTime::now();
                        }
                    }
                }
            }
        }else{
            let server_attempt = server::parse_slice(&self.buf[0..size]);

            if let Some(a) = server_attempt {
                if let server::PacketType::Handshake(_, dat) = a {
                    self.handle_client2server_handshake(dat, size, addr, client_map);
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
            //if let RemoteClientWrapper::Client(client) = client {
                let outgoing = client.get_outgoing_packets();
                if outgoing.len() == 0 { continue; }
                
                if let Some(udp) = self.get_udp_client(client) {
                    if udp.srv_addr != self.local_addr {
                        continue;
                    }

                    for packet in outgoing {
                            let result = self.socket.send_to(packet.as_slice(), udp.last_addr);
                            if let Err(v) = result { // TODO: prettier error handling
                                                        // TODO: dont remove this packet, retry next time around
                                                        // TODO: kill connection if never gets through? for example it might have disconnected from wifi, related to TODO 1
                                println!("Failed to send packet: {}", v);
                            }else if let Ok(v) = result {
                                println!("Send {} packets to {}! (from {})", v, udp.last_addr, self.local_addr);
                            }
                    }
                }
            //}
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