pub mod packet_types;
pub mod types;
pub mod ids;
mod tests;

pub use packet_types::*;
pub use types::*;

use deku::DekuContainerRead;
use bytes::Buf;

#[allow(dead_code)]
pub fn parse_buf(bb: &mut dyn Buf) -> Option<PacketType> {
	let result = PacketType::from_bytes((bb.chunk(), 0));

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
