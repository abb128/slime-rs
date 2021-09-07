use deku::prelude::*;

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