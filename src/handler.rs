use std::collections::VecDeque;
use std::{collections::HashMap, net::SocketAddr};

use crate::packet_parsing::{client, server, types};


use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct TrackerData {
    pub buf_rotations: HashMap<u32, types::Quaternion>,
    pub last_heartbeat: SystemTime,

    pub last_vibrate: SystemTime
}

pub trait ClientServer {
    fn receive(&mut self); // receives and processes incoming packets
    fn flush(&mut self);   // flushes outgoing packets

    fn get_trackers(&self) -> Vec<&TrackerData>; // gets trackers
}

pub trait ClientHandler {
    fn get_data_mut(&mut self) -> &mut TrackerData;
    fn get_data(&self) -> &TrackerData;

    fn queue_packet(&mut self, data: Vec<u8>);

    fn send_packet(&mut self, packet: client::PacketType) {
        let encoded = client::to_bytes(&packet);
        if let Some(bytes) = encoded {
            self.queue_packet(bytes);
        }
    }

    fn update_heartbeat_time(&mut self) {
        self.get_data_mut().last_heartbeat = SystemTime::now();
    }

    fn update_rotation(&mut self, id: u32, rotation: types::Quaternion) {
        self.get_data_mut().buf_rotations.insert(id, rotation);
    }

    fn get_rotation(&self, id: u32) -> Option<&types::Quaternion> {
        self.get_data().buf_rotations.get(&id)
    }
    
    fn update_rotation_advanced(&mut self, sid: types::SensorID, data: server::RotationDataType){
        let types::SensorID(id) = sid;
        match data {
            server::RotationDataType::Normal(q, inf) => self.update_rotation(id as u32, q),
            server::RotationDataType::Correction(q, inf) => todo!(),
        }
    }

    fn handle_handshake(&mut self, data: types::HandshakeData) {
        // TODO eat the handshake data
        
        // respond with stuf
        let response = client::PacketType::Handshake(
            client::ClientHandshake {
                version: '5' as u8
            }
        );

        self.send_packet(response);
    }

    fn receive_packet(&mut self, packet: server::PacketType) {
        // any packet means the connection is still alive, so we
        // update anyway
        self.update_heartbeat_time();

        match packet {
            server::PacketType::Heartbeat(_) => {},
            server::PacketType::Rotation(_, q) => self.update_rotation(0, q),
            server::PacketType::Gyroscope(_, _) => {},
            server::PacketType::Handshake(_, h) => self.handle_handshake(h),
            server::PacketType::Accelerometer(_, _) => {},
            server::PacketType::Magnetometer(_, _) => {},
            server::PacketType::RawCalibration(_, _) => todo!(),
            server::PacketType::GyroCalibration(_, _) => todo!(),
            server::PacketType::Config(_, _) => todo!(),
            server::PacketType::RawMagnetometer(_, _) => todo!(),
            server::PacketType::Ping(_, _) => todo!(),
            server::PacketType::Serial(_, _) => todo!(),
            server::PacketType::Battery(_, _) => {},
            server::PacketType::Tap(_, _, _) => todo!(),
            server::PacketType::ResetReason(_, _) => todo!(),
            server::PacketType::SensorInfo(_, _, _) => todo!(),
            server::PacketType::Rotation2(_, q) => self.update_rotation(1, q),
            server::PacketType::RotationData(_, i, t) => todo!(),
            server::PacketType::MagnetometerAccuracy(_, _, _) => todo!(),
        }
    }
}


use std::net::UdpSocket;

pub struct UdpServer {
    clients: HashMap<types::MacAddress, UdpClient>,
    addr_to_mac : HashMap<SocketAddr, types::MacAddress>,
    socket: UdpSocket,
    buf: [u8; 256],
}


#[derive(Debug)]
struct UdpClient {
    addr: SocketAddr,
    mac: types::MacAddress,
    pub tracker: TrackerData,
    queued_packets: VecDeque<Vec<u8>>
}



impl Default for TrackerData {
    fn default() -> Self {
        Self { buf_rotations: HashMap::default(), last_heartbeat: UNIX_EPOCH, last_vibrate: UNIX_EPOCH }
    }
}


impl UdpServer {
    fn handle_existing_packet(&mut self, pkt: server::PacketType, addr: SocketAddr) {
        if let Some(mac) = self.addr_to_mac.get(&addr) {
            if let Some(client) = self.clients.get_mut(&mac) {
                client.receive_packet(pkt);
            }
        }
    }

    fn handle_newcomer(&mut self, handshake: types::HandshakeData, addr: SocketAddr) {
        self.addr_to_mac.insert(addr.clone(), handshake.mac_address.clone());
        let mac_id = handshake.mac_address.clone();

        let mut client = UdpClient::new(addr, handshake.mac_address.clone());
        println!("New client {:?}", client);


        client.receive_packet(server::PacketType::Handshake(types::PacketID(0), handshake));


        self.clients.insert(mac_id, client);
    }

    pub fn new() -> UdpServer {
        // TODO: DO NOT USE EXPECT!!
        let cl = UdpServer {
            socket: UdpSocket::bind("0.0.0.0:6969").expect("Port is busy"),
            buf: [0u8; 256],
            clients: HashMap::default(),
            addr_to_mac: HashMap::default()
        };

        cl.socket.set_nonblocking(true).expect("Couldn't set to non-blocking!");
        
        cl
    }
}

impl ClientServer for UdpServer {

    fn receive(&mut self) {
        loop {
            let r = self.socket.recv_from(&mut self.buf);
            match r {
                Ok((size, addr)) => {
                    let dec = server::parse_slice(&self.buf[0..size]);
                    if let Some(pkt) = dec {
                        match pkt {
                            server::PacketType::Handshake(_, h) => {
                                self.handle_newcomer(h, addr)
                            },
                            _ => {
                                self.handle_existing_packet(pkt, addr);
                            }
                        }
                    }
                }
                Err(_) => break
            }
        }
    }

    fn flush(&mut self) {
        for client in self.clients.values_mut() {
            client.queued_packets.push_back(
                client::to_bytes(
                    &client::PacketType::Other(client::OPacketType::Heartbeat(
                        client::HeartbeatToClient {
                            extra: 0
                        }
                    ))
                ).expect("asd")
            );
            if SystemTime::now().duration_since(client.tracker.last_vibrate).expect("asd") > Duration::from_millis(1500){
                client.tracker.last_vibrate = SystemTime::now();
                client.queued_packets.push_back(
                    client::to_bytes(
                        &client::PacketType::Other(client::OPacketType::Vibrate(
                            client::VibrateData {
                                duration_seconds: 0.1,
                                frequency: 1.0,
                                amplitude: 1.0,
                            }
                        ))
                    ).expect("asd")
                );
            }
            while let Some(pkt) = client.queued_packets.get(0) {
                let result = self.socket.send_to(pkt.as_slice(),
                    client.addr);
                
                match result {
                    Ok(_) => {
                        client.queued_packets.pop_front();
                    },
                    Err(e) => {
                        println!("Failed to send packet: {:?}", e)
                    }
                }
            }
        }
    }

    fn get_trackers(&self) -> Vec<&TrackerData> {
        let mut trackers: Vec<&TrackerData> = vec![];
        for client in self.clients.values() {
            trackers.push(&client.tracker);
        }

        return trackers;
    }
}

impl ClientHandler for UdpClient {
    fn get_data_mut(&mut self) -> &mut TrackerData {
        return &mut self.tracker;
    }

    fn get_data(&self) -> &TrackerData {
        return &self.tracker;
    }

    fn queue_packet(&mut self, data: Vec<u8>) {
        self.queued_packets.push_back(data);
    }
}

impl UdpClient {
    fn new(addr: SocketAddr, mac: types::MacAddress) -> UdpClient {
        UdpClient {
            addr: addr,
            mac: mac,
            tracker: Default::default(),
            queued_packets: Default::default(),
        }
    }
}

/*

    fn poll_for_updates(&mut self) -> Vec<server::PacketType> {
        let mut packets: Vec<server::PacketType> = vec![];
        loop {
            let r = self.socket.recv_from(&mut self.buf);
            match r {
                Ok((size, addr)) => {
                    self.last_addr = Some(addr.clone());
                    let dec = server::parse_slice(&self.buf[0..size]);
                    if let Some(packet) = dec {
                        packets.push(packet);
                    }
                }
                Err(_) => break
            }
        }

        packets
    }*/