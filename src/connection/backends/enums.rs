
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BackendType {
    Udp
}


use super::udp::UdpClient;
#[derive(Debug)]
pub enum BackendDataRef<'a> {
    Udp(&'a UdpClient)
}

#[derive(Debug)]
pub enum BackendDataMutRef<'a> {
    Udp(&'a mut UdpClient)
}


pub trait BackendRemoteData: core::fmt::Debug {
    fn get_data(&self) -> BackendDataRef<'_>;
    fn get_data_mut(&mut self) -> BackendDataMutRef<'_>;
}
