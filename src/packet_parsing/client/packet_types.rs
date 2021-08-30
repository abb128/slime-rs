use crate::packet_parsing::client::ids;
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


// TODO
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[deku(type = "u32")]
pub enum CommandType {
    #[deku(id = "0")]
    Calibrate,
    #[deku(id = "1")]
    SendConfig,
    #[deku(id = "2")]
    Blink
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct HeartbeatToClient {
    pub extra: u8 // usually 0
}

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

    // TODO
    #[deku(id = "ids::CONFIG")]
    Config,

    #[deku(id = "ids::PING")]
    Ping,

    #[deku(id = "ids::SENSOR_INFO")]
    SensorInfo
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