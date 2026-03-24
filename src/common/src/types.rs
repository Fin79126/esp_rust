// shared/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BattCommand {
    pub servo: [u16; 4], // 0-180など
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MainCommand {
    Setup,
    GetStatus,
    AttachServo,
    DetachServo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ESPStatus {
    pub heartbeat: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LegsCommand {
    pub servo: [u16; 12], // 0-180など
    pub bldc: [i16; 2],
}
