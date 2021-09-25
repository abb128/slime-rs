use std::net::SocketAddr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BackendType {
    Udp(SocketAddr)
}





use crate::connection::listener::Listener;

use super::udp::{UdpClient, UdpServer};
#[derive(Debug)]
pub enum BackendDataRef<'a> {
    Udp(&'a UdpClient)
}



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


#[derive(Debug)]
pub enum BackendDataMutRef<'a> {
    Udp(&'a mut UdpClient)
}


pub trait BackendRemoteData: core::fmt::Debug {
    fn get_data(&self) -> BackendDataRef<'_>;
    fn get_data_mut(&mut self) -> BackendDataMutRef<'_>;
}
