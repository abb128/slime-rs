#![deny(rust_2018_idioms)]
#![deny(rust_2018_compatibility)]

mod packet_parsing;
mod connection;
mod tracker;
mod types;

use std::time::SystemTime;

use connection::backends::udp::UdpServer;

use crate::connection::{listener::*};

fn main() -> std::io::Result<()> {
    let mut collection = ListenerCollection::default();

    for p in 6969u16..7011u16 {
        let udp = UdpServer::new(p);
        if let Ok(srv) = udp {
            collection.add_server(Box::new(srv));
        }else{
            println!("Failed to register port {}", p);
            panic!();
        }
    }

    let mut last_print = SystemTime::now();
    loop {
        collection.receive();
        collection.flush();

        let curr_print = SystemTime::now();

        if curr_print.duration_since(last_print).unwrap().as_millis() > 500 {
            let mut i = 0;
            for client in collection.clients() {
                println!("[{}]: {:?}", i, client.get_tracker().sensors);
                i = i + 1;
            }

            last_print = curr_print;
        }
    }
}