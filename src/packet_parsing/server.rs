// Packets received by the SERVER 
// (sent by the client)

pub mod packet_types;
pub mod ids;
mod tests;

pub use packet_types::*;

use deku::{DekuContainerRead, DekuContainerWrite};
use bytes::Buf;

#[allow(dead_code)]
pub fn parse_slice(b: &[u8]) -> Option<PacketType> {
	let result = PacketType::from_bytes((b, 0));

	match result {
		Ok((_rest, result)) => {
			Some(result)
		}
		Err(_) => {
			None
		}
	}
}

#[allow(dead_code)]
pub fn parse_buf(bb: &mut dyn Buf) -> Option<PacketType> {
    parse_slice(bb.chunk())
}

#[allow(dead_code)]
pub fn to_bytes(pk: &PacketType) -> Option<Vec<u8>> {
    let result = pk.to_bytes();

    match result {
		Ok(data) => {
			Some(data)
		}
		Err(_) => {
			None
		}
	}
}