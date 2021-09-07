use std::{collections::HashMap};
use crate::packet_parsing::{client, server, types::{self, MacAddress, Quaternion, SensorID}};
use std::time::{SystemTime};
use crate::tracker::TrackerData;

use super::backends::enums::*;

#[derive(Debug)]
pub struct Client {
    clients: HashMap<BackendType, Box<dyn BackendRemoteData>>,
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

    pub fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        Some(self.clients.get_mut(ctype)?.get_data_mut())
    }

    pub fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        Some(self.clients.get(ctype)?.get_data())
    }
    
    pub fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, &str> {
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


