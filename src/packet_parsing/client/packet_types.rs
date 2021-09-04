use crate::packet_parsing::{client::ids, types::{DeviceConfig, Matrix3x3, PingId, SensorID, Vector}};
//use crate::packet_parsing::types::*;
use deku::prelude::*;


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[deku(magic = b"Hey OVR =D ")]
pub struct ClientHandshake {
    pub version: u8 // ASCII char
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct VibrateData {
    pub duration_seconds: f32,
    pub frequency: f32,
    pub amplitude: f32
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[deku(type = "u32")]
pub enum CommandType {
    #[deku(id = "1")]
    Calibrate,
    #[deku(id = "2")]
    SendConfig,
    #[deku(id = "3")]
    Blink
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct HeartbeatToClient {
    pub extra: u8 // usually 0
}


pub type SensorStateNotified = bool;


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[deku(magic = b"\0", type="u16")]
pub enum OPacketType {
    #[deku(id = "ids::HEARTBEAT")]
	Heartbeat(HeartbeatToClient),

    #[deku(id = "ids::VIBRATE")]
	Vibrate(VibrateData),

    
    #[deku(id = "ids::COMMAND")]
	Command(CommandType),

    #[deku(id = "ids::CONFIG")]
    Config(DeviceConfig),

    #[deku(id = "ids::PING")]
    Ping(PingId),

    #[deku(id = "ids::SENSOR_INFO")]
    SensorInfo(SensorID, SensorStateNotified)
}




// This needs to be done like this to support handshake properly
// (handshake expects first byte to be 3)
// An alternative would be to have the u32 "\03Hey"
// as its ID
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
#[deku(type = "u8")]
pub enum PacketType {
    #[deku(id = "ids::F_HANDSHAKE")]
	Handshake(ClientHandshake),

    #[deku(id = "ids::F_OTHER")]
    Other(OPacketType)
}