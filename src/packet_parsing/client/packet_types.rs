use crate::packet_parsing::ids;
use crate::packet_parsing::*;
use deku::prelude::*;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
#[deku(type = "u32")]
pub enum PacketType {
    #[deku(id = "ids::HEARTBEAT")]
	Heartbeat(PacketID),
}