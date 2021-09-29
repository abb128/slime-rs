// A server (PC, Quest, etc) that connects to a client (us)
// Sends client::PacketType to us
// We send server::PacketType to them

// TODO: proper implementation
// TODO: have heartbeat

use super::*;
use super::super::backends::enums::*;
use crate::packet_parsing::{server, client, types::*};

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
            client::PacketType::Handshake(hnd) => self.handle_handshake(hnd),
            client::PacketType::Other(o) => match o {
                client::OPacketType::Heartbeat(_) => todo!(),
                client::OPacketType::Vibrate(_) => todo!(),
                client::OPacketType::Command(_) => todo!(),
                client::OPacketType::Config(_) => todo!(),
                client::OPacketType::Ping(_) => todo!(),
                client::OPacketType::SensorInfo(_, _) => todo!(),
            }
        }
    }

    pub fn send_handshake_to_server(&mut self) {
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

    pub fn handle_handshake(&mut self, hnd: client::ClientHandshake) {
        println!("Server handshake, version {}", hnd.version - ('0' as u8));
    }
}

impl ClientsContainer for Server {
    fn find_client_type_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        self.remote.find_client_type_mut(ctype)
    }

    fn find_client_type<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        self.remote.find_client_type(ctype)
    }

    fn insert_client_type(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, ClientInsertionFailure> {
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

impl RemoteClientContainer for Server {
    fn get_remote_client(&self) -> &RemoteClient {
        &self.remote
    }

    fn get_remote_client_mut(&mut self) -> &mut RemoteClient {
        &mut self.remote
    }
}