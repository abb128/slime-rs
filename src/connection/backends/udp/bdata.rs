use std::net::SocketAddr;
use std::time::SystemTime;

use super::super::enums::*;

#[derive(Debug)]
pub struct UdpClient {
    pub(super) srv_addr: SocketAddr,
    pub(super) last_addr: SocketAddr,
    pub(super) last_activity: SystemTime
}

impl BackendRemoteData for UdpClient {
    fn get_data(&self) -> BackendDataRef<'_> {
        BackendDataRef::Udp(self)
    }

    fn get_data_mut(&mut self) -> BackendDataMutRef<'_> {
        BackendDataMutRef::Udp(self)
    }

    fn is_alive(&self) -> bool {
        let curr_time = SystemTime::now();

        let duration_since_last = curr_time.duration_since(self.last_activity);
        if let Ok(duration) = duration_since_last {
            return duration.as_secs() < 8;
        }

        return false;
    }
}