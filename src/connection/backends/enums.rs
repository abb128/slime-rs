use std::net::SocketAddr;
use super::udp::{UdpClient, UdpServer};
use crate::connection::listener::Listener;


// BackendType is used as a key in a hashmap to index different backend
// listeners
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BackendType {
    Udp(SocketAddr)
}


// BackendListeners actually implement the network logic (dealing with UDP,
// Bluetooth, or whatever). They must implement Listener trait and be added to
// the Listener impl below.
pub enum BackendListener {
    Udp(UdpServer)
}


impl Listener for BackendListener {
    fn receive(&mut self, client_map: &mut crate::connection::listener::RemoteMap) {
        match self {
            BackendListener::Udp(a) => a.receive(client_map)
        }
    }

    fn flush(&mut self, client_map: &mut crate::connection::listener::RemoteMap) {
        match self {
            BackendListener::Udp(a) => a.flush(client_map)
        }
    }
}


// BackendData contains arbitrary state info necessary for listeners to
// communicate with clients. For example, might contain the UDP socket address

#[derive(Debug)]
pub enum BackendDataMutRef<'a> {
    Udp(&'a mut UdpClient)
}

#[derive(Debug)]
pub enum BackendDataRef<'a> {
    Udp(&'a UdpClient)
}


pub trait BackendRemoteData: core::fmt::Debug {
    fn get_data(&self) -> BackendDataRef<'_>;
    fn get_data_mut(&mut self) -> BackendDataMutRef<'_>;
}
