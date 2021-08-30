#![allow(dead_code)]

pub const F_OTHER               : u8 =  0;
pub const F_HANDSHAKE           : u8 =  3;

pub const HEARTBEAT             :u16 =	1;
pub const VIBRATE               :u16 =	2;
pub const COMMAND               :u16 =  4;


// shared with packet_parsing::server::ids
pub const CONFIG                :u16 =  8;
pub const PING                  :u16 =  10;
pub const SENSOR_INFO           :u16 =  15;