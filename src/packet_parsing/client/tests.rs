#[cfg(test)]
mod parse_tests {
    use crate::packet_parsing::client::parse_slice;
	use crate::packet_parsing::client::parse_buf;
	use crate::packet_parsing::client::ids;
	use crate::packet_parsing::client::*;
	use bytes::{BytesMut, BufMut, Buf};


	fn assert_encoded_is_equal(buf: &BytesMut, parsed: &PacketType){
		let encoded = to_bytes(&parsed).unwrap();

		assert!(buf.remaining() > 0, 
			"This test is invalid - the original buffer is blank");

		for i in 0..buf.remaining() {
			assert_eq!(encoded[i], buf[i]);
		}
	}

	#[test]
	fn test_parse_handshake(){
		let mut buf = BytesMut::with_capacity(64);
		buf.put_u8(ids::F_HANDSHAKE);
		buf.put_u8('H' as u8);
        buf.put_u8('e' as u8);
        buf.put_u8('y' as u8);
        buf.put_u8(' ' as u8);
        buf.put_u8('O' as u8);
        buf.put_u8('V' as u8);
        buf.put_u8('R' as u8);
        buf.put_u8(' ' as u8);
        buf.put_u8('=' as u8);
        buf.put_u8('D' as u8);
        buf.put_u8(' ' as u8);
        buf.put_u8('5' as u8);

		let parsed_packet = parse_buf(&mut buf).unwrap();
		assert_eq!(parsed_packet,
			PacketType::Handshake(
				ClientHandshake {
                    version: '5' as u8
                }
			)
		);
		let parsed_packet2 = parse_slice(buf.chunk()).unwrap();
		assert_eq!(parsed_packet2,
			PacketType::Handshake(
				ClientHandshake {
                    version: '5' as u8
                }
			)
		);
		assert_encoded_is_equal(&buf, &parsed_packet);
		assert_encoded_is_equal(&buf, &parsed_packet2);
	}


    #[test]
	fn test_parse_vibrate(){
		let mut buf = BytesMut::with_capacity(64);
		buf.put_u32(ids::VIBRATE as u32);
		buf.put_f32(5.912f32);
        buf.put_f32(0.01f32);
        buf.put_f32(0.058f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();
		assert_eq!(parsed_packet,
			PacketType::Other(
                OPacketType::Vibrate(
                    VibrateData {
                        duration_seconds: 5.912f32,
                        frequency: 0.01f32,
                        amplitude: 0.058f32
                    }
                )
			)
		);

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

}
