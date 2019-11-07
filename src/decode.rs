use std::ops::Sub;

const GYRO_FACTOR: f64 = 1E-4;
const ACCEL_FACTOR: f64 = 1E-5;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Off,
    Sensor,
    Firmware,
    Calibrate,
    KeepAlive,
    UnknownSetting,
    LPMEnable,
    LPMDisable,
    VRMode,
}

impl Command {
    pub fn value(&self) -> [u8; 2] {
        use Command::*;
        match *self {
            Off => [0, 0],
            Sensor => [1, 0],
            Firmware => [2, 0],
            Calibrate => [3, 0],
            KeepAlive => [4, 0],
            UnknownSetting => [5, 0],
            LPMEnable => [6, 0],
            LPMDisable => [7, 0],
            VRMode => [8, 0],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Axis {
    pub x: i16,
    pub y: i16,
}

impl Axis {
    pub const ZERO: Self = Self{ x: 0, y: 0};
}

impl From<[u8; 3]> for Axis {
    fn from(value: [u8; 3]) -> Self {
        let a = value[0] as u16;
        let b = value[1] as u16;
        let c = value[2] as u16;
        Self {
            x: ((((a & 0xF) << 6) + ((b & 0xFC) >> 2)) & 0x3FF) as i16,
            y: ((((b & 0x3) << 8) + ((c & 0xFF) >> 0)) & 0x3FF) as i16,
        }
    }
}

impl Sub for Axis {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Buttons {
    trigger: bool,
    home: bool,
    back: bool,
    touchpad: bool,
    volume_up: bool,
    volume_down: bool,
}

impl From<u8> for Buttons {
    fn from(n: u8) -> Self {
        Self {
            trigger: (n & (1 << 0)) != 0,
            home: (n & (1 << 1)) != 0,
            back: (n & (1 << 2)) != 0,
            touchpad: (n & (1 << 3)) != 0,
            volume_up: (n & (1 << 4)) != 0,
            volume_down: (n & (1 << 5)) != 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub axis: Axis,
    pub timestamp: i32,
    pub temperature: u8,
    pub buttons: Buttons,
}

impl<'a> From<&'a [u8]> for Packet {
    fn from(data: &'a [u8]) -> Self {
        Self {
            axis: Axis::from([data[54], data[55], data[56]]),
            timestamp: i32::from_le_bytes([data[0], data[1], data[2], data[3]]) & (-1),
            temperature: data[57],
            buttons: Buttons::from(data[58]),
        }
    }
}
