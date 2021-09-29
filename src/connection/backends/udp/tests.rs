#[cfg(test)]
mod udp_tests {
    use std::str::FromStr;

    use super::super::*;

    fn obtain_server(id: u8) -> UdpServer {
        let server = UdpServer::new(16000+(id as u16));
        if let Ok(srv) = server {
            return srv;
        }else{
            panic!("Failed to bind UDP server to port {}. Please ensure the port range 16000-16255 isn't being used", 16000+(id as u16));
        }
    }

    fn obtain_blank_remote_map() -> RemoteMap {
        Default::default()
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
        match bdata {
            BackendDataRef::Udp(udp) => {
                assert_eq!(udp.srv_addr, srv.local_addr, "The server address of the UdpClient should match the true server address");
                assert_eq!(udp.last_addr, addr, "The address of the client should match its true address.");
            },
            _ => {
                panic!("Queried BackendType::Udp, but got a non-UDP bdata");
            }
        }
    }
}