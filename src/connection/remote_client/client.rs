// A client (phone, tracker) that connects to the SlimeVR server (local)
// Sends server::PacketType to us
// We send client::PacketType to them

// TOOD: Send heartbeat

use super::*;
use super::super::backends::enums::*;

use std::time::{SystemTime};
use crate::tracker::TrackerData;
use crate::packet_parsing::{client, server, types::*};

#[derive(Debug)]
pub struct Client {
    remote: RemoteClient,
    tracker: TrackerData
}

impl Client {
    pub fn new(data: &HandshakeData) -> Client {
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

    pub fn handle_handshake(&mut self, _h: HandshakeData) {
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
                let BatteryData(f) = lvl;
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

impl RemoteClientContainer for Client {
    fn get_remote_client(&self) -> &RemoteClient {
        &self.remote
    }

    fn get_remote_client_mut(&mut self) -> &mut RemoteClient {
        &mut self.remote
    }
}