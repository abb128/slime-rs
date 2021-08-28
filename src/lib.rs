mod packet_parsing;

use bytes::{BufMut, BytesMut};

#[test]
fn example(){
	
	let mut buf = BytesMut::with_capacity(12);

	buf.put_u32(packet_parsing::ids::HEARTBEAT);
	buf.put_u64(7u64);

	let result = packet_parsing::parse_buf(&mut buf).unwrap();

	assert_eq!(result, packet_parsing::PacketType::Heartbeat(
		packet_parsing::PacketID(7u64)
	));
}