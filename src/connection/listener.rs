use std::{collections::HashMap};
use crate::packet_parsing::types::MacAddress;

use super::client::*;


pub type RemoteMap = HashMap<MacAddress, RemoteClientWrapper>;

#[derive(Default)]
pub struct ListenerCollection {
    listeners: Vec<Box<dyn Listener>>,
    remotes: RemoteMap
}

pub trait Listener {
    fn receive(&mut self, client_map: &mut RemoteMap); // receives and processes incoming packets
    fn flush(&mut self, client_map: &mut RemoteMap);   // flushes buffered outgoing packets
}

impl ListenerCollection {
    pub fn clients(&mut self) -> std::collections::hash_map::Values<'_, MacAddress, RemoteClientWrapper> {
        let values = self.remotes.values().into_iter();
        return values;
    }

    pub fn receive(&mut self) {
        for server in &mut self.listeners {
            server.receive(&mut self.remotes);
        }
    }

    pub fn flush(&mut self) {
        for server in &mut self.listeners {
            server.flush(&mut self.remotes);
        }

        for (_, remote) in &mut self.remotes {
            remote.clear_outgoing();
        }
    }

    pub fn add_server(&mut self, server: Box<dyn Listener>) {
        self.listeners.push(server);
    }
}