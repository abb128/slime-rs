#[cfg(test)]
mod udp_tests {
    use core::time;
    use std::str::FromStr;

    use crate::packet_parsing::types::Quaternion;

    use super::super::*;

    fn id_to_port(id: u8) -> u16 { 16000 + (id as u16) }

    fn obtain_server(id: u8) -> UdpServer {
        let server = UdpServer::new(id_to_port(id));
        if let Ok(srv) = server {
            return srv;
        }else{
            panic!("Failed to bind UDP server to port {}. Please ensure the port range 16000-16255 isn't being used", id_to_port(id));
        }
    }

    fn obtain_blank_remote_map() -> RemoteMap {
        Default::default()
    }

    fn ensure_received(srv: &mut UdpServer, map: &mut RemoteMap, tgt_map_len: usize) -> bool {
        let mut connected = false;
        for _ in 0..5 {
            srv.receive(map);
            if tgt_map_len != 0 && map.len() == tgt_map_len {
                connected = true;
                break;
            }

            // Should be instant, but sleep just in case
            std::thread::sleep(time::Duration::from_millis(10));
        }

        return connected;
    }

    #[test]
    fn test_constructoring(){
        let srv = obtain_server(0);

        assert_eq!(srv.addr_to_mac.len(), 0);
        assert_eq!(srv.local_addr, SocketAddr::from_str("0.0.0.0:16000").unwrap());

        let map = obtain_blank_remote_map();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_connecting_to_server(){
        let mut srv = obtain_server(1);
        let mut map = obtain_blank_remote_map();

        let addr = SocketAddr::from_str("127.0.0.3:50182").unwrap();
        srv.connect_to_server(addr, &mut map);

        assert_eq!(srv.addr_to_mac.len(), 1, "An address to mac mapping should exist upon attempt to connect");
        assert_eq!(map.len(), 1, "The remote map should contain a single client upon attempt to connect");

        let (addr2, mac) = srv.addr_to_mac.iter().next().unwrap();
        assert_eq!(addr, addr2.clone(), "The address of the mapping should match");

        let (mac2, client) = map.iter().next().unwrap();
        assert_eq!(mac.clone(), mac2.clone(), "The mac addresses should match between addr_to_map and remotemap");


        let remote_server: &Server;

        match client {
            RemoteClientWrapper::Client(_) => panic!("Tried to connect to server, but UDP has given us client instead.."),
            RemoteClientWrapper::Server(w) => {
                remote_server = w;
            }
        }

        let outgoing = remote_server.get_outgoing_packets();
        assert_eq!(outgoing.len(), 1, "An outgoing handshake packet should be present");

        let pkt = server::parse_slice(outgoing.iter().next().unwrap()).unwrap();
        match pkt {
            server::PacketType::Handshake(_, _) => {},
            _ => {
                panic!("First outgoing packet was not a handshake packet")
            }
        }

        let bdata = remote_server.find_bdata(&BackendType::Udp(srv.local_addr)).expect("Backend data should exist in new UDP client");
        let udp_ref;
        match bdata {
            BackendDataRef::Udp(udp) => {
                assert_eq!(udp.srv_addr, srv.local_addr, "The server address of the UdpClient should match the true server address");
                assert_eq!(udp.last_addr, addr, "The address of the client should match its true address.");

                udp_ref = udp;
            },
            _ => {
                panic!("Queried BackendType::Udp, but got a non-UDP bdata");
            }
        }

        let udp_ref2 = srv.get_udp_client(remote_server).expect("UDPServer should be able to find the UdpClient");

        assert_eq!(udp_ref as *const UdpClient, udp_ref2 as *const UdpClient, "The reference given by get_udp_client should match finding the bdata manually");
    }

    #[test]
    fn test_self_connection(){
        let mut client_srv = obtain_server(2);
        let mut client_map = obtain_blank_remote_map();

        let mut server_srv = obtain_server(3);
        let mut server_map = obtain_blank_remote_map();

        let server_addr = SocketAddr::from_str("127.0.0.1:16003").unwrap();
        client_srv.connect_to_server(server_addr, &mut client_map);

        client_srv.flush(&mut client_map);

        assert!(ensure_received(&mut server_srv, &mut server_map, 1),
            "Connecting from local client to local server should succeed");

        let (_srv_mac, srv_client) = server_map.iter_mut().next().unwrap();

        if let RemoteClientWrapper::Client(_c) = srv_client {
            // success
        }else{
            panic!("A Client should've connected, but got a non-client");
        }
    }

    #[test]
    fn test_self_connection_send_rotation(){
        let mut client_srv = obtain_server(4);
        let mut client_map = obtain_blank_remote_map();

        let mut server_srv = obtain_server(5);
        let mut server_map = obtain_blank_remote_map();

        let server_addr = SocketAddr::from_str("127.0.0.1:16005").unwrap();
        client_srv.connect_to_server(server_addr, &mut client_map);


        for i in 0..16 {
            let tgt_quat = Quaternion {
                x: 0.2 + (i as f32)/32.0,
                y: 0.333 - (i as f32)/32.0,
                z: 0.31415926 + (i as f32)/32.0,
                w: 1.5837 - (i as f32)/32.0
            };


            client_srv.flush(&mut client_map);

            assert!(ensure_received(&mut server_srv, &mut server_map, 1),
                "Connecting from local client to local server should succeed");


            let (_cli_mac, cli_server) = client_map.iter_mut().next().unwrap();
            if let RemoteClientWrapper::Server(s) = cli_server {
                s.send_packet(&server::PacketType::Rotation(128, tgt_quat));
            }else{
                panic!("The server was meant to be connecting to a server, but found non-server");
            }


            client_srv.flush(&mut client_map);
            ensure_received(&mut server_srv, &mut server_map, 0);

            let (_srv_mac, srv_client) = server_map.iter_mut().next().unwrap();

            if let RemoteClientWrapper::Client(c) = srv_client {
                // success
                
                let tracker = c.get_tracker();
                assert_eq!(tracker.sensors.len(), 1, "A single default sensor should exist after 1 rotation packet");

                let (_id, sensor) = tracker.sensors.iter().next().unwrap();

                assert_eq!(sensor.last_quat, tgt_quat, "The received sensor's rotation quat should match the one sent");
            }else{
                panic!("A Client should've connected, but got a non-client");
            }
        }
    }
}