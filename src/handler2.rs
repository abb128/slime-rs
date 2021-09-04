use std::{collections::HashMap, net::SocketAddr};
use crate::packet_parsing::{client, server, types::{self, MacAddress, Quaternion, SensorID}};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Sensor {
    pub last_quat: Quaternion
}

impl Default for Sensor {
    fn default() -> Self {
        Self {
                last_quat: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0
            }
        }
    }
}


#[derive(Debug)]
pub struct TrackerData {
    pub sensors: HashMap<SensorID, Sensor>,
    pub last_heartbeat: SystemTime,
}


impl TrackerData {
    fn get_sensor_or_default(&mut self, id: SensorID) -> &mut Sensor {
        return self.sensors.entry(id).or_insert(Sensor::default())
    }

    fn update_rotation(&mut self, id: SensorID, quat: Quaternion){
        self.get_sensor_or_default(id).last_quat = quat;
    }
}


impl Default for TrackerData {
    fn default() -> Self {
        Self { sensors: Default::default(), last_heartbeat: SystemTime::now() }
    }
}

#[derive(Debug)]
pub struct Client {
    clients: HashMap<ClientType, ClientTypeData>,
    outgoing_buf: Vec<client::PacketType>,

    tracker: TrackerData,
    mac: MacAddress
}

impl Client {
    pub fn new(data: &types::HandshakeData) -> Client {
        Client {
            clients: Default::default(),
            tracker: TrackerData::default(),
            outgoing_buf: Default::default(),
            mac: data.mac_address.clone()
        }
    }

    pub fn send_packet(&mut self, pkt: client::PacketType) {
        self.outgoing_buf.push(pkt);
    }
    
    pub fn get_outgoing_packets(&self) -> &Vec<client::PacketType> {
        &self.outgoing_buf
    }
    
    pub fn clear_outgoing(&mut self) {
        self.outgoing_buf.clear();
    }

    pub fn handle_handshake(&mut self, _h: types::HandshakeData) {
        let response = client::PacketType::Handshake(
            client::ClientHandshake {
                version: '5' as u8
            }
        );

        self.send_packet(response);
    }

    fn update_rotation(&mut self, id: SensorID, quat: Quaternion){
        self.tracker.update_rotation(id, quat)
    }

    fn update_heartbeat_time(&mut self) {
        self.tracker.last_heartbeat = SystemTime::now();
    }

    pub fn find_client_type_mut(&mut self, ctype: &ClientType) -> Option<&mut ClientTypeData> {
        return self.clients.get_mut(ctype);
    }

    pub fn find_client_type(&self, ctype: &ClientType) -> Option<&ClientTypeData> {
        return self.clients.get(ctype);
    }
    
    pub fn insert_client_type(&mut self, t: ClientType, d: ClientTypeData) -> Result<&mut ClientTypeData, &str> {
        if let Some(a) = self.clients.get(&t) {
            return Result::Err("Client type already exists!");
        }
        let entry = self.clients.entry(t);
        
        return Ok(entry.or_insert(d));
    }

    pub fn receive_packet(&mut self, pkt: server::PacketType) {
        self.update_heartbeat_time();
        // TODO: use packet id...

        match pkt {
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
            server::PacketType::Battery(_, lvl) => {
                let types::BatteryData(f) = lvl;
                println!("Battery {}", f);
            },
            server::PacketType::Tap(_, _, _) => todo!(),
            server::PacketType::ResetReason(_, _) => todo!(),
            server::PacketType::SensorInfo(_, _, _) => todo!(),
            server::PacketType::Rotation2(_, q) => self.update_rotation(1, q),
            server::PacketType::RotationData(_, i, t) => todo!(),
            server::PacketType::MagnetometerAccuracy(_, _, _) => todo!(),
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ClientType {
    Udp
}


use crate::udpserver::UdpClient;
#[derive(Debug)]
pub enum ClientTypeData {
    Udp(UdpClient)
}

pub type ClientMap = HashMap<MacAddress, Client>;

#[derive(Default)]
pub struct ServerCollection {
    servers: Vec<Box<dyn ClientServer>>,
    clients: ClientMap
}

pub trait ClientServer {
    fn receive(&mut self, client_map: &mut ClientMap); // receives and processes incoming packets
    fn flush(&mut self, client_map: &mut ClientMap);   // flushes buffered outgoing packets
}

impl ServerCollection {
    pub fn clients(&mut self) -> std::collections::hash_map::Values<'_, MacAddress, Client> {
        let values = self.clients.values().into_iter();
        return values;
    }

    pub fn receive(&mut self) {
        for server in &mut self.servers {
            server.receive(&mut self.clients);
        }
    }

    pub fn flush(&mut self) {
        for server in &mut self.servers {
            server.flush(&mut self.clients);
        }
    }

    pub fn add_server(&mut self, server: Box<dyn ClientServer>) {
        self.servers.push(server);
    }
}