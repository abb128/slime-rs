use std::{collections::HashMap, time::SystemTime};

use crate::types::*;

#[derive(Debug)]
pub struct Sensor {
    pub last_quat: Quaternion
}

impl Default for Sensor {
    fn default() -> Self {
        Self {
                last_quat: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0
            }
        }
    }
}


#[derive(Debug)]
pub struct TrackerData {
    pub sensors: HashMap<SensorID, Sensor>,
    pub last_heartbeat: SystemTime,
}


impl TrackerData {
    pub fn get_sensor_or_default(&mut self, id: SensorID) -> &mut Sensor {
        return self.sensors.entry(id).or_insert(Sensor::default())
    }

    pub fn update_rotation(&mut self, id: SensorID, quat: Quaternion){
        self.get_sensor_or_default(id).last_quat = quat;
    }
}


impl Default for TrackerData {
    fn default() -> Self {
        Self { sensors: Default::default(), last_heartbeat: SystemTime::now() }
    }
}
