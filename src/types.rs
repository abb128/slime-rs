mod mac_address;
mod math;

pub use mac_address::*;
pub use math::*;


use deku::prelude::*;




pub type SensorID = i8;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct BatteryData(pub f32);




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

