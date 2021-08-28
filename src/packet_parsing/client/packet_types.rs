use crate::packet_parsing::client::ids;
use crate::packet_parsing::types::*;
use deku::prelude::*;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
#[deku(type = "u32")]
pub enum PacketType {
    #[deku(id = "ids::HEARTBEAT")]
	Heartbeat(PacketID),
}