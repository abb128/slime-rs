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
