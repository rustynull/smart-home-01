#![allow(dead_code)]
#![allow(unused_variables)]
/// Socket:
/// provide text representation
/// turn on
/// turn off
/// provide current power consumption info
///  
/// Thermometer:
/// provide current temperature measurements
use std::{fmt::Display, time::Duration};

// //////// Socket ////////

enum SocketError {
    // Different error types
}

enum SocketState {
    On,
    Off,
}

struct Socket {
    desc: String,         // description
    state: SocketState,   // current state
    uptime: Duration,     // time since power on
    average_current: u32, // average current consumption
    average_voltage: u32, // average voltage
}

impl Socket {
    fn new(desc: &str) -> Self {
        todo!()
    }
    /// Turn socket on
    fn turn_on(&self) -> Result<(), SocketError> {
        todo!()
    }
    /// Turn socket off
    fn turn_off(&self) -> Result<(), SocketError> {
        todo!()
    }
    /// Returns current power consumption
    fn power_consuption(&self) -> Result<u32, SocketError> {
        todo!()
    }
}

// String representation of a socket
impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

////////////////////////////////

// //////// Thermometer ////////

enum ThermometerError {}

enum ThermometerState {
    On,
    Off,
}

struct Thermometer {
    desc: String,            // description
    state: ThermometerState, // state
}

impl Thermometer {
    fn new(desc: &str) -> Self {
        todo!()
    }
    fn get_temperature(&self) -> Result<u32, ThermometerError> {
        todo!()
    }
}

////////////////////////////////
