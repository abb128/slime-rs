use std::fmt::Write;

use deku::prelude::*;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Matrix3x3 {
	pub a00: f32,
	pub a01: f32,
	pub a02: f32,

	pub a10: f32,
	pub a11: f32,
	pub a12: f32,

	pub a20: f32,
	pub a21: f32,
	pub a22: f32
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ImuInfo(pub i32, pub i32, pub i32);

impl Default for ImuInfo {
	fn default() -> ImuInfo {
		ImuInfo(0, 0, 0)
	}
}


#[derive(PartialEq, Clone, Eq, Hash, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct MacAddress(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);



impl ToString for MacAddress {
	fn to_string(&self) -> String {
		let MacAddress(a, b, c, d, e, f) = self;

		format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
			a, b, c, d, e, f)
	}
}

impl std::fmt::Debug for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Default for MacAddress {
	fn default() -> MacAddress {
		MacAddress(0, 0, 0, 0, 0, 0)
	}
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
// TODO CRITICAL this aint null terminated...
pub struct StringWithLength {
	#[deku(update = "self.str_data.len()")]
	str_len: u8,

	#[deku(count = "str_len")]
	str_data: Vec<u8>
}

impl ToString for StringWithLength {
	fn to_string(&self) -> String {
		// TODO: DO NOT USE EXPECT!!!
		String::from_utf8(self.str_data.to_vec()).expect("Invalid UTF-8")
	}
}

impl Default for StringWithLength {
	fn default() -> StringWithLength {
		let default_str = "";
		StringWithLength {
			str_len: 0,
			str_data: default_str.as_bytes().to_vec()
		}
	}
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct FirmwareString(pub StringWithLength);

impl ToString for FirmwareString {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

impl Default for FirmwareString {
	fn default() -> FirmwareString {
		let default_str = "owoTrack";
		FirmwareString(StringWithLength {
			str_len: 0,
			str_data: default_str.as_bytes().to_vec()
		})
	}
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct HandshakeData {
	#[deku(cond = "deku::rest.len() >= 4", default = "0")]
	pub board_type: i32,
	#[deku(cond = "deku::rest.len() >= 4", default = "0")]
	pub imu_type: i32,
	#[deku(cond = "deku::rest.len() >= 4", default = "0")]
	pub mcu_type: i32,

	#[deku(cond = "deku::rest.len() >= 4*3")]
	pub imu_info: ImuInfo,

	#[deku(cond = "deku::rest.len() >= 4", default = "0")]
	pub firmware_build: i32,
	
	#[deku(cond = "deku::rest.len() >= 1")]
	pub firmware: FirmwareString,
	
	#[deku(cond = "deku::rest.len() >= 6")]
	pub mac_address: MacAddress
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RawCalibrationData(
    pub i32, pub i32, pub i32, pub i32, pub i32, pub i32
);

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct GyroCalibrationData(pub f32, pub f32, pub f32);

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct SerialData(pub StringWithLength);

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct BatteryData(pub f32);

pub type PingId = i32;
pub type SensorID = i8;
pub type PacketID = u64;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TapData(pub i8);

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ResetReasonData(pub i8);

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct SensorInfoData {
	pub status: i8
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct MagnetometerAccuracyData(pub f32);


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct CalibrationInfo(pub i8);



// TODO: data gets reversed?? wtf
// https://github.com/SlimeVR/SlimeVR-Server/blob/12d7f191ee4b737281cc2e3b04f01366bc67197c/src/main/java/io/eiren/vr/trackers/MPUTracker.java#L63
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[allow(non_snake_case)]
pub struct CalibrationConfig {
    pub accel_B: Vector,
    pub accel_Ainv: Matrix3x3,

    pub mag_B: Vector,
    pub mag_Ainv: Matrix3x3,

    pub gyro_off: Vector
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct DeviceConfig {
    pub calibration: CalibrationConfig,

    pub device_id: i32,
    pub device_mode: i32
}