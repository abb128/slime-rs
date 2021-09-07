#![deny(rust_2018_idioms)]

mod packet_parsing;
mod connection;
mod tracker;
mod types;

use connection::backends::udp::UdpServer;

use crate::connection::{listener::*};

fn main() -> std::io::Result<()> {
    let mut collection = ListenerCollection::default();

    let mut udp = UdpServer::new(6969).unwrap();
    collection.add_server(Box::new(udp));

    loop {
        collection.receive();
        collection.flush();

        for client in collection.clients() {
            println!("{:?}", client)
        }
    }

    /*
    let mut server = UdpServer::new(6969).expect("Failed to create UDP Server");

    loop {
        server.receive();
        server.flush();

        /*
        let mut clients = server.get_trackers_mut();
        for i in 0..clients.len() {
            println!("[{}] rotation: {:?}", i, clients[i].get_data().buf_rotations.get(&0));


            if SystemTime::now().duration_since(clients[i].get_data().last_vibrate).expect("asd") > Duration::from_millis(1000){
                clients[i].get_data_mut().last_vibrate = SystemTime::now();
                clients[i].send_packet(
                        client::PacketType::Other(client::OPacketType::Vibrate(
                            client::VibrateData {
                                duration_seconds: 0.1,
                                frequency: 1.0,
                                amplitude: 1.0,
                            }
                        ))
                );
            }
        }*/


        thread::sleep(Duration::from_millis(32u64));
    }
    */
    Ok(())
}