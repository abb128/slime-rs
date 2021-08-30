use std::{collections::HashMap, net::SocketAddr};

use crate::packet_parsing::{client, server, types};




pub trait ClientHandler { // TODO this should use slices in functions? not packettype
    fn poll_for_updates(&mut self) -> Vec<server::packet_types::PacketType>;
    fn send_packet(&self, packet: client::packet_types::PacketType);

    fn update_heartbeat_time(&mut self);
    fn update_rotation(&mut self, id: u32, rotation: types::Quaternion);
    fn get_rotation(&self, id: u32) -> Option<&types::Quaternion>;
    
    fn update_rotation_advanced(&mut self, sid: types::SensorID, data: server::packet_types::RotationDataType){
        let types::SensorID(id) = sid;
        match data {
            server::RotationDataType::Normal(q, inf) => self.update_rotation(id as u32, q),
            server::RotationDataType::Correction(q, inf) => todo!(),
        }
    }

    fn handle_handshake(&self, data: types::HandshakeData) {
        // TODO eat the handshake data
        
        // respond with stuf
        let response = client::packet_types::PacketType::Handshake(
            client::packet_types::ClientHandshake {
                version: '5' as u8
            }
        );

        self.send_packet(response);
    }

    fn receive_packet(&mut self, packet: server::packet_types::PacketType) {
        match packet {
            server::PacketType::Heartbeat(_) => self.update_heartbeat_time(),
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

pub struct UdpClient<> {
    socket: UdpSocket, // TODO a socket should handle many clients, not just 1
    buf: [u8; 256],
    last_addr: Option<SocketAddr>,

    data: ClientData
}

#[derive(Default)]
struct ClientData {
    buf_rotations: HashMap<u32, types::Quaternion>
}


impl UdpClient<> {
    pub fn new() -> UdpClient<> {
        let cl = UdpClient {
            socket: UdpSocket::bind("0.0.0.0:6969").expect("Port is busy"),
            buf: [0u8; 256],
            last_addr: None,
            data: ClientData::default()
        };

        cl.socket.set_nonblocking(true).expect("Couldn't set to non-blocking!");
        
        cl
    }
}

impl ClientHandler for UdpClient<> {
    fn send_packet(&self, packet: client::packet_types::PacketType) {
        match self.last_addr {
            Some(addr) => {
                let bytes = client::to_bytes(&packet).expect("Cant to bytes!");
                self.socket.send_to(bytes.as_slice(), addr);
            }
            None => {
                println!("Bro no address cant send!! {:?}", packet);
            }
        }
    }

    fn update_heartbeat_time(&mut self) {
    }

    fn update_rotation(&mut self, id: u32, rotation: types::Quaternion) {
        self.data.buf_rotations.insert(id, rotation);
    }

    fn poll_for_updates(&mut self) -> Vec<server::packet_types::PacketType> {
        let mut packets: Vec<server::packet_types::PacketType> = vec![];
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
    }

    fn get_rotation(&self, id: u32) -> Option<&types::Quaternion> {
        self.data.buf_rotations.get(&id)
    }
}