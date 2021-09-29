// RemoteClient describes a generic remote connection with another device
//
// The device may be either a client or a server. Their own respective structs
// provide their own separate logic for various packet types.
//
// They wrap around RemoteClient, which provides some simple useful fields that 
// any remote connection would need, such as a BackendData map, outgoing packet
// buffer and a unique MAC address for identification (not guaranteed to be an
// actual valid mac address)
//
// The RemoteClientWrapper enum lists the various remote connections that can
// exist.

pub mod client;
pub mod server;

pub use client::*;
pub use server::*;

use super::backends::enums::*;
use crate::packet_parsing::types::*;
use std::{collections::HashMap};

#[derive(Debug)]
pub enum BDataInsertionFailure {
    AlreadyExists
}

pub trait BDataContainer {
    fn find_bdata_mut<'a>(&'a mut self, btype: &BackendType) -> Option<BackendDataMutRef<'a>>;
    fn find_bdata<'a>(&'a self, btype: &BackendType) -> Option<BackendDataRef<'a>>;
    fn insert_bdata(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, BDataInsertionFailure>;
}

pub trait PacketBuffered {
    fn send_packet(&mut self, pkt: Vec<u8>);
    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>>;
    fn clear_outgoing(&mut self);
}


#[derive(Debug)]
pub struct RemoteClient {
    clients: HashMap<BackendType, Box<dyn BackendRemoteData>>,
    outgoing_buf: Vec<Vec<u8>>,
    mac: MacAddress
}



impl RemoteClient {
    pub fn new(mac: MacAddress) -> RemoteClient {
        RemoteClient {
            clients: Default::default(),
            outgoing_buf: Default::default(),
            mac
        }
    }

}

impl PacketBuffered for RemoteClient {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.outgoing_buf.push(pkt);
    }
    
    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        &self.outgoing_buf
    }
    
    fn clear_outgoing(&mut self) {
        self.outgoing_buf.clear();
    }
}

impl BDataContainer for RemoteClient {
    fn find_bdata_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        Some(self.clients.get_mut(ctype)?.get_data_mut())
    }

    fn find_bdata<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        Some(self.clients.get(ctype)?.get_data())
    }
    
    fn insert_bdata(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, BDataInsertionFailure> {
        if let Some(_) = self.clients.get(&t) {
            return Err(BDataInsertionFailure::AlreadyExists);
        }
        let entry = self.clients.entry(t);
        
        return Ok(entry.or_insert(d));
    }
}



pub trait RemoteClientContainer {
    fn get_remote_client(&self) -> &RemoteClient;
    fn get_remote_client_mut(&mut self) -> &mut RemoteClient;
}


#[derive(Debug)]
pub enum RemoteClientWrapper {
    Client(Client),
    Server(Server)
}



impl RemoteClientContainer for RemoteClientWrapper {
    fn get_remote_client(&self) -> &RemoteClient {
        match self {
            RemoteClientWrapper::Client(client) => client.get_remote_client(),
            RemoteClientWrapper::Server(server) => server.get_remote_client()
        }
    }

    fn get_remote_client_mut(&mut self) -> &mut RemoteClient {
        match self {
            RemoteClientWrapper::Client(client) => client.get_remote_client_mut(),
            RemoteClientWrapper::Server(server) => server.get_remote_client_mut()
        }
    }
}


impl BDataContainer for RemoteClientWrapper {
    fn find_bdata_mut<'a>(&'a mut self, ctype: &BackendType) -> Option<BackendDataMutRef<'a>> {
        self.get_remote_client_mut().find_bdata_mut(ctype)
    }

    fn find_bdata<'a>(&'a self, ctype: &BackendType) -> Option<BackendDataRef<'a>> {
        self.get_remote_client().find_bdata(ctype)
    }

    fn insert_bdata(&mut self, t: BackendType, d: Box<dyn BackendRemoteData>) -> Result<&mut Box<dyn BackendRemoteData>, BDataInsertionFailure> {
        self.get_remote_client_mut().insert_bdata(t, d)
    }
}

impl PacketBuffered for RemoteClientWrapper {
    fn send_packet(&mut self, pkt: Vec<u8>) {
        self.get_remote_client_mut().send_packet(pkt)
    }

    fn get_outgoing_packets(&self) -> &Vec<Vec<u8>> {
        self.get_remote_client().get_outgoing_packets()
    }

    fn clear_outgoing(&mut self) {
        self.get_remote_client_mut().clear_outgoing()
    }
}