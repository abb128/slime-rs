#![deny(rust_2018_idioms)]
#![deny(rust_2018_compatibility)]

mod packet_parsing;
mod connection;
mod tracker;
mod types;

use std::{any::Any, env, net::SocketAddr, str::FromStr, time::SystemTime};

use connection::{backends::{enums::BackendListener, udp::UdpServer}, remote_client::RemoteClientWrapper};
use types::Quaternion;

use crate::{connection::{listener::*}, packet_parsing::server};


fn run_client_example(ip: String) -> Option<()> {
    let mut collection = ListenerCollection::default();

    let mut udp = UdpServer::new(65000).unwrap();
    collection.add_server(BackendListener::Udp(udp));


    let mut last_print = SystemTime::now();
    let mut id = 0;

    let listener = &mut collection.listeners[0];

    if let BackendListener::Udp(udp) = listener {
        udp.connect_to_server(SocketAddr::from_str(&ip).expect("Invalid IP address"), &mut collection.remotes);
    }
    

    loop {
        collection.receive();
        collection.flush();

        let curr_print = SystemTime::now();

        if curr_print.duration_since(last_print).unwrap().as_millis() > 500 {
            let mut i = 0;
            for client in collection.clients_mut() {
                if let RemoteClientWrapper::Server(srv) = client {
                    println!("[{}]: {:?}", i, srv);

                    id += 1;
                    srv.send_packet(&server::PacketType::Rotation(id, Quaternion {
                        x: 1.1,
                        y: (id as f32)*2.0,
                        z: 1.1,
                        w: 1.1
                    }));
                }
                i = i + 1;
            }

            last_print = curr_print;
        }
    }

    return None;
}


fn run_server_example() -> Option<()> {
    let mut collection = ListenerCollection::default();

    for p in 6969u16..7011u16 {
        let udp = UdpServer::new(p);
        if let Ok(srv) = udp {
            collection.add_server(BackendListener::Udp(srv));
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
                if let RemoteClientWrapper::Client(client) = client {
                    println!("[{}]: {:?}", i, client.get_tracker().sensors);
                }
                i = i + 1;
            }

            last_print = curr_print;
        }
    }
}


fn main() -> std::io::Result<()> {
    let mut args = env::args();

    let _exe_name = args.next();

    if let Some(s) = args.next() {
        if s == "server" {
            run_server_example();
            return Ok(())
        }else if s == "client" {
            run_client_example(args.next().expect("You need to supply IP"));//println!("Sorry");
            return Ok(())
        }
    }

    println!("Supply an argument: [server/client]");
    

    Ok(())
}