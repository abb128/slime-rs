#![allow(dead_code)]

pub const HEARTBEAT             :u32 =	1;
pub const VIBRATE               :u32 =	2;
pub const HANDSHAKE             :u32 =  3;
pub const COMMAND               :u32 =  4;


// shared with packet_parsing::server::ids
pub const CONFIG                :u32 =  8;
pub const PING                  :u32 =  10;
pub const SENSOR_INFO           :u32 =  15;