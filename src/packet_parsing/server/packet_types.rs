use crate::packet_parsing::server::ids;
use crate::packet_parsing::types::*;
use deku::prelude::*;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[deku(type = "u8")]
pub enum RotationDataType {
	#[deku(id = "1")]
	Normal(Quaternion, CalibrationInfo),

	#[deku(id = "2")]
	Correction(Quaternion, CalibrationInfo)
}


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
#[deku(type = "u32")]
pub enum PacketType {
    #[deku(id = "ids::HEARTBEAT")]
	Heartbeat(PacketID),

    #[deku(id = "ids::ROTATION")]
	Rotation(PacketID, Quaternion),

    #[deku(id = "ids::GYROSCOPE")]
	Gyroscope(PacketID, Vector),

    #[deku(id = "ids::HANDSHAKE")]
	Handshake(PacketID, HandshakeData),

    #[deku(id = "ids::ACCELEROMETER")]
	Accelerometer(PacketID, Vector),

    #[deku(id = "ids::MAGNETOMETER")]
	Magnetometer(PacketID, Vector),
    
    #[deku(id = "ids::RAW_CALIBRATION_DATA")]
	RawCalibration(PacketID, RawCalibrationData),

    #[deku(id = "ids::GYRO_CALIBRATION_DATA")]
	GyroCalibration(PacketID, GyroCalibrationData),

    #[deku(id = "ids::CONFIG")]
	Config(PacketID, ConfigurationData),

    #[deku(id = "ids::RAW_MAGNETOMETER")]
	RawMagnetometer(PacketID, Vector),

    #[deku(id = "ids::PING")]
	Ping(PacketID, PingId),

    #[deku(id = "ids::SERIAL")]
	Serial(PacketID, SerialData),
    
    #[deku(id = "ids::BATTERY")]
	Battery(PacketID, BatteryData),

    #[deku(id = "ids::TAP")]
	Tap(PacketID, SensorID, TapData),

    #[deku(id = "ids::RESET_REASON")]
	ResetReason(PacketID, ResetReasonData),

    #[deku(id = "ids::SENSOR_INFO")]
	SensorInfo(PacketID, SensorID, SensorInfoData),

    #[deku(id = "ids::ROTATION_2")]
	Rotation2(PacketID, Quaternion),

	#[deku(id = "ids::ROTATION_DATA")]
	RotationData(PacketID, SensorID, RotationDataType),
    
    #[deku(id = "ids::MAGNETOMETER_ACCURACY")]
	MagnetometerAccuracy(PacketID, SensorID, MagnetometerAccuracyData),
}