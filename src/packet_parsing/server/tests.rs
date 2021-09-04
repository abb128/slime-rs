#[cfg(test)]
mod parse_tests {
    use crate::packet_parsing::server::parse_slice;
	use crate::packet_parsing::server::parse_buf;
	use crate::packet_parsing::server::ids;
	use crate::packet_parsing::server::*;
	use crate::packet_parsing::types::*;
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
	fn test_parse_packet_id_and_heartbeat(){
		for i in 0u64..64u64 {
			let mut buf = BytesMut::with_capacity(12);

			buf.put_u32(ids::HEARTBEAT);
			buf.put_u64(i * 7u64);

			let parsed_packet = parse_buf(&mut buf).unwrap();

			assert_eq!(parsed_packet,
				PacketType::Heartbeat(
					(i * 7u64) as PacketID
				)
			);


			let parsed_packet2 = parse_slice(buf.chunk()).unwrap();

			assert_eq!(parsed_packet2,
				PacketType::Heartbeat(
					(i * 7u64) as PacketID
				)
			);

			assert_encoded_is_equal(&buf, &parsed_packet);
			assert_encoded_is_equal(&buf, &parsed_packet2);
		}
	}

	#[test]
	fn test_parse_rotation(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ROTATION);
		buf.put_u64(64u64);

		buf.put_f32(5.12f32);
		buf.put_f32(1.28f32);
		buf.put_f32(9.185f32);
		buf.put_f32(10.582f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Rotation(
			(64u64) as PacketID,
			Quaternion {
				x: 5.12f32,
				y: 1.28f32,
				z: 9.185f32,
				w: 10.582f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_gyroscope(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::GYROSCOPE);
		buf.put_u64(64u64);

		buf.put_f32(5.12f32);
		buf.put_f32(1.28f32);
		buf.put_f32(9.185f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Gyroscope(
			(64u64) as PacketID,
			Vector {
				x: 5.12f32,
				y: 1.28f32,
				z: 9.185f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_basic_handshake(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::HANDSHAKE);
		buf.put_u64(0u64);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		match &parsed_packet {
			PacketType::Handshake(_, h) => {
				assert_eq!(h.board_type, 0);
				assert_eq!(h.imu_type, 0);
				assert_eq!(h.mcu_type, 0);
				assert_eq!(h.imu_info, ImuInfo(0, 0, 0));
				assert_eq!(h.firmware_build, 0);
				assert_eq!(h.firmware.to_string(), "owoTrack");
				assert_eq!(h.mac_address.to_string(), "00:00:00:00:00:00");
			}

			_ => {
				panic!("Handshake data was not detected as handshake!");
			}
		}

		assert_encoded_is_equal(&buf, &parsed_packet);
	}
	
	#[test]
	fn test_parse_slime_handshake(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::HANDSHAKE);
		buf.put_u64(0u64);


		buf.put_i32(259i32);
		buf.put_i32(175i32);
		buf.put_i32(38i32);


		buf.put_i32(1i32);
		buf.put_i32(100i32);
		buf.put_i32(10000i32);

		buf.put_i32(999372i32);

		buf.put_u8(8);
		buf.put_u8('H' as u8);
		buf.put_u8('e' as u8);
		buf.put_u8('l' as u8);
		buf.put_u8('l' as u8);
		buf.put_u8('o' as u8);
		buf.put_u8('d' as u8);
		buf.put_u8('e' as u8);
		buf.put_u8('r' as u8);


		buf.put_u8(0x10);
		buf.put_u8(0x20);
		buf.put_u8(0x30);
		buf.put_u8(0xFF);
		buf.put_u8(0xEE);
		buf.put_u8(0xDA);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		match &parsed_packet{
			PacketType::Handshake(_, h) => {
				assert_eq!(h.board_type, 259i32);
				assert_eq!(h.imu_type, 175i32);
				assert_eq!(h.mcu_type, 38i32);
				assert_eq!(h.imu_info, ImuInfo(1, 100, 10000));
				assert_eq!(h.firmware_build, 999372i32);
				assert_eq!(h.firmware.to_string(), "Helloder");
				assert_eq!(h.mac_address.to_string(), "10:20:30:FF:EE:DA");
			}

			_ => {
				panic!("Handshake data was not detected as handshake!");
			}
		}

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_accelerometer(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ACCELEROMETER);
		buf.put_u64(64u64);

		buf.put_f32(5824.12f32);
		buf.put_f32(57578.32f32);
		buf.put_f32(1578925.85f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Accelerometer(
			(64u64) as PacketID,
			Vector {
				x: 5824.12f32,
				y: 57578.32f32,
				z: 1578925.85f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_magnetometer(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::MAGNETOMETER);
		buf.put_u64(64u64);

		buf.put_f32(5824.12f32);
		buf.put_f32(57578.32f32);
		buf.put_f32(1578925.85f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Magnetometer(
			(64u64) as PacketID,
			Vector {
				x: 5824.12f32,
				y: 57578.32f32,
				z: 1578925.85f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_buf_calibration(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::RAW_CALIBRATION_DATA);
		buf.put_u64(64u64);

		buf.put_i32(10i32);
		buf.put_i32(100i32);
		buf.put_i32(500i32);
		buf.put_i32(600i32);
		buf.put_i32(700i32);
		buf.put_i32(800i32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::RawCalibration(
			(64u64) as PacketID,
			RawCalibrationData(
				10i32,
				100i32,
				500i32,
				600i32,
				700i32,
				800i32
			)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_gyro_calibration(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::GYRO_CALIBRATION_DATA);
		buf.put_u64(64u64);

		buf.put_f32(0.10f32);
		buf.put_f32(0.100f32);
		buf.put_f32(0.500f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::GyroCalibration(
			(64u64) as PacketID,
			GyroCalibrationData(
				0.1f32,
				0.1f32,
				0.5f32
			)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_buf_magnetometer(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::RAW_MAGNETOMETER);
		buf.put_u64(64u64);

		buf.put_f32(0.10f32);
		buf.put_f32(0.100f32);
		buf.put_f32(0.500f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::RawMagnetometer(
			(64u64) as PacketID,
			Vector {
				x: 0.1f32,
				y: 0.1f32,
				z: 0.5f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_ping(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::PING);
		buf.put_u64(64u64);

		buf.put_i32(90000i32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Ping(
			(64u64) as PacketID,
			(90000i32) as PingId
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_serial(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::SERIAL);
		buf.put_u64(64u64);

		buf.put_u8(10);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);
		buf.put_u8('A' as u8);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		match &parsed_packet {
			PacketType::Serial(_, data) => {
				let SerialData(s) = data;
				assert_eq!(s.to_string(), "AAAAAAAAAA");
			}

			_ => panic!("Serial was not parsed as serial!")
		}

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_battery(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::BATTERY);
		buf.put_u64(64u64);

		buf.put_f32(99.99f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Battery(
			(64u64) as PacketID,
			BatteryData(99.99f32)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_tap(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::TAP);
		buf.put_u64(64u64);

		buf.put_i8(3);
		buf.put_u8(10);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Tap(
			(64u64) as PacketID,
			(3) as SensorID,
			TapData(10)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}


	#[test]
	fn test_parse_reset_reason(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::RESET_REASON);
		buf.put_u64(64u64);

		buf.put_i8(-32i8);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::ResetReason(
			(64u64) as PacketID,
			ResetReasonData(-32i8)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_sensor_info(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::SENSOR_INFO);
		buf.put_u64(64u64);

		buf.put_i8(4i8);

		buf.put_i8(-32i8);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::SensorInfo(
			(64u64) as PacketID,
			(4i8) as SensorID,
			SensorInfoData {
				status: -32i8
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_rotation2(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ROTATION_2);
		buf.put_u64(64u64);

		buf.put_f32(10.20f32);
		buf.put_f32(59.57832f32);
		buf.put_f32(3.141592653f32);
		buf.put_f32(0.001f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::Rotation2(
			(64u64) as PacketID,
			Quaternion {
				x: 10.20f32,
				y: 59.57832f32,
				z: 3.141592653f32,
				w: 0.001f32
			}
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_rotation_normal(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ROTATION_DATA);
		buf.put_u64(64u64);

		buf.put_u8(3u8); // sensor id

		buf.put_u8(1u8); // data type

		buf.put_f32(10.20f32);
		buf.put_f32(59.57832f32);
		buf.put_f32(3.141592653f32);
		buf.put_f32(0.001f32);

		buf.put_u8(90u8); // calibration info

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::RotationData(
			(64u64) as PacketID,
			(3i8) as SensorID,
			RotationDataType::Normal(
				Quaternion {
					x: 10.20f32,
					y: 59.57832f32,
					z: 3.141592653f32,
					w: 0.001f32
				},
				CalibrationInfo(90i8)
			)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_rotation_correction(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ROTATION_DATA);
		buf.put_u64(64u64);

		buf.put_u8(3u8); // sensor id

		buf.put_u8(2u8); // data type

		buf.put_f32(10.20f32);
		buf.put_f32(59.57832f32);
		buf.put_f32(3.141592653f32);
		buf.put_f32(0.001f32);

		buf.put_u8(90u8); // calibration info

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::RotationData(
			(64u64) as PacketID,
			(3i8) as SensorID,
			RotationDataType::Correction(
				Quaternion {
					x: 10.20f32,
					y: 59.57832f32,
					z: 3.141592653f32,
					w: 0.001f32
				},
				CalibrationInfo(90i8)
			)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_mag_acc(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::MAGNETOMETER_ACCURACY);
		buf.put_u64(64u64);

		buf.put_i8(58i8);
		buf.put_f32(-500.5f32);

		let parsed_packet = parse_buf(&mut buf).unwrap();

		assert_eq!(parsed_packet, PacketType::MagnetometerAccuracy(
			(64u64) as PacketID,
			(58i8) as SensorID,
			MagnetometerAccuracyData(-500.5f32)
		));

		assert_encoded_is_equal(&buf, &parsed_packet);
	}

	#[test]
	fn test_parse_malformed_packet(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_f64(158.8f64);
		buf.put_f64(158.8f64);
		buf.put_f64(158.8f64);


		let parsed_packet = parse_buf(&mut buf);

		assert_eq!(parsed_packet, None);
	}

	#[test]
	fn test_parse_rotation_malformed(){
		let mut buf = BytesMut::with_capacity(128);

		buf.put_u32(ids::ROTATION_DATA);
		buf.put_u64(64u64);

		buf.put_i8(-1i8); // sensor id

		buf.put_u8(90u8); // data type

		buf.put_f32(10.20f32);
		buf.put_f32(59.57832f32);
		buf.put_f32(3.141592653f32);
		buf.put_f32(0.001f32);

		buf.put_u8(90u8); // calibration info

		let parsed_packet = parse_buf(&mut buf);

		assert_eq!(parsed_packet, None);
	}
}
