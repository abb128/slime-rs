use std::{collections::HashMap};
use crate::packet_parsing::{client::{self, ClientHandshake}, server, types::{self, FirmwareString, HandshakeData, ImuInfo, MacAddress, Quaternion, SensorID}};
use std::time::{SystemTime};
use crate::tracker::TrackerData;

use super::backends::enums::*;


pub trait ClientsContainer {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>>;
    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>>;
    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str>;
}

pub trait PacketBuffered {
    fn send_packet(&mut self, pkt: Vec<u8>);
    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>>;
    fn clear_outgoing(&mut self);
}


#[derive(Debug)]
pub struct RemoteClient {
    clients: HashMap<BackendType, Box<dyn BackendRemoteData>>,
    outgoing_buf: Vec<Vec<u8>>,
    mac: MacAddress
}



impl RemoteClient {
    pub fn new(mac: MacAddress) -> RemoteClient {
        RemoteClient {
            clients: Default::default(),
            outgoing_buf: Default::default(),
            mac
        }
    }

}

impl PacketBuffered for RemoteClient {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.outgoing_buf.push(pkt);
    }
    
    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        &self.outgoing_buf
    }
    
    fn clear_outgoing(&mut self) {
        self.outgoing_buf.clear();
    }
}

impl ClientsContainer for RemoteClient {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        Some(self.clients.get_mut(ctype)?.get_data_mut())
    }

    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        Some(self.clients.get(ctype)?.get_data())
    }
    
    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str> {
        if let Some(_) = self.clients.get(&t) {
            return Result::Err("Client type already exists!");
        }
        let entry = self.clients.entry(t);
        
        return Ok(entry.or_insert(d));
    }
}


// A client (phone, tracker) that connects to the SlimeVR server
// Sends server::PacketType to us
// We send client::PacketType to them
#[derive(Debug)]
pub struct Client {
    remote: RemoteClient,
    tracker: TrackerData
}

// TOOD: Send heartbeat

impl Client {
    pub fn new(data: &types::HandshakeData) -> Client {
        Client {
            tracker: TrackerData::default(),
            remote: RemoteClient::new(data.mac_address.clone())
        }
    }

    pub fn send_packet(&mut self, pkt: &client::PacketType) {
        if let Some(bytes) = client::to_bytes(pkt) {
            self.remote.send_packet(bytes);
        }
    }


    pub fn get_tracker(&self) -> &TrackerData {
        &self.tracker
    }

    pub fn handle_handshake(&mut self, _h: types::HandshakeData) {
        let response = client::PacketType::Handshake(
            client::ClientHandshake {
                version: '5' as u8
            }
        );

        self.send_packet(&response);
    }

    fn update_rotation(&mut self, id: SensorID, quat: Quaternion){
        self.tracker.update_rotation(id, quat)
    }

    fn update_heartbeat_time(&mut self) {
        self.tracker.last_heartbeat = SystemTime::now();
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
            server::PacketType::RotationData(_, _, _) => todo!(),
            server::PacketType::MagnetometerAccuracy(_, _, _) => todo!(),
        }
    }
}

impl ClientsContainer for Client {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        self.remote.find_client_type_mut(ctype)
    }

    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        self.remote.find_client_type(ctype)
    }

    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str> {
        self.remote.insert_client_type(t, d)
    }
}

impl PacketBuffered for Client {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.remote.send_packet(pkt)
    }

    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        self.remote.get_outgoing_packets()
    }

    fn clear_outgoing(&mut self) {
        self.remote.clear_outgoing()
    }
}




// A client (phone, tracker) that connects to the SlimeVR server
// Sends client::PacketType to us
// We send server::PacketType to them
// TODO: proper implementation
// TODO: have heartbeat
#[derive(Debug)]
pub struct Server {
    remote: RemoteClient,
}

impl Server {
    pub fn new(mac: MacAddress) -> Server {
        return Server { remote: RemoteClient::new(mac) }
    }

    pub fn send_packet(&mut self, pkt: &server::PacketType) {
        if let Some(bytes) = server::to_bytes(pkt) {
            self.remote.send_packet(bytes);
        }
    }

    pub fn receive_packet(&mut self, pkt: client::PacketType) {
        match pkt {
            client::PacketType::Handshake(h) => println!("{:?}", h),
            client::PacketType::Other(o) => todo!(),
        }
    }

    pub fn do_handshake(&mut self) {
        let req = server::PacketType::Handshake(
            0,

            // TODO: some way to get this data so we dont just provide invalid
            HandshakeData {
                board_type: 0,
                imu_type: 0,
                mcu_type: 0,
                imu_info: ImuInfo(0, 0, 0),
                firmware_build: 0,
                firmware: FirmwareString::default(),
                mac_address: MacAddress(0, 0, 0, 0, 0, 0)
            }
        );

        self.send_packet(&req);
    }

    pub fn handle_handshake(&mut self, hnd: ClientHandshake) {
        println!("Server handshake, version {}", hnd.version);
    }
}

impl ClientsContainer for Server {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        self.remote.find_client_type_mut(ctype)
    }

    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        self.remote.find_client_type(ctype)
    }

    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str> {
        self.remote.insert_client_type(t, d)
    }
}

impl PacketBuffered for Server {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.remote.send_packet(pkt)
    }

    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        self.remote.get_outgoing_packets()
    }

    fn clear_outgoing(&mut self) {
        self.remote.clear_outgoing()
    }
}




#[derive(Debug)]
pub enum RemoteClientWrapper {
    Client(Client),
    Server(Server)
}

impl RemoteClientWrapper {
    fn get_remote_client(&self) -> &RemoteClient {
        match self {
            RemoteClientWrapper::Client(client) => &client.remote,
            RemoteClientWrapper::Server(server) => &server.remote
        }
    }

    fn get_remote_client_mut(&mut self) -> &mut RemoteClient {
        match self {
            RemoteClientWrapper::Client(client) => &mut client.remote,
            RemoteClientWrapper::Server(server) => &mut server.remote
        }
    }
}

impl ClientsContainer for RemoteClientWrapper {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        self.get_remote_client_mut().find_client_type_mut(ctype)
    }

    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        self.get_remote_client().find_client_type(ctype)
    }

    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str> {
        self.get_remote_client_mut().insert_client_type(t, d)
    }
}

impl PacketBuffered for RemoteClientWrapper {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.get_remote_client_mut().send_packet(pkt)
    }

    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        self.get_remote_client().get_outgoing_packets()
    }

    fn clear_outgoing(&mut self) {
        self.get_remote_client_mut().clear_outgoing()
    }
}