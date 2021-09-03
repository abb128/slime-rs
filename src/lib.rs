mod packet_parsing;

#[allow(unused_imports)]
use bytes::{BufMut, BytesMut};

#[test]
fn example(){
	
	let mut buf = BytesMut::with_capacity(12);

	buf.put_u32(packet_parsing::server::ids::HEARTBEAT);
	buf.put_u64(7u64);

	let result = packet_parsing::server::parse_buf(&mut buf).unwrap();

	assert_eq!(result, packet_parsing::server::PacketType::Heartbeat(
		packet_parsing::types::PacketID(7u64)
	));
}