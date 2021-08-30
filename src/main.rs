
mod packet_parsing;
mod handler;

use std::{thread, time::Duration};

use crate::handler::*;

fn main() -> std::io::Result<()> {
    let mut server = UdpServer::new();

    loop {
        server.receive();
        server.flush();


        let trackers = server.get_trackers();
        for i in 0..trackers.len() {
           println!("[{}] rotation: {:?}", i, trackers[i].buf_rotations.get(&0));
        }

        thread::sleep(Duration::from_millis(64u64));
    }

    Ok(())
}