use std::convert::TryFrom;
use std::fs::File;
use std::io;

use arrayvec::{ArrayString, ArrayVec};
use uhid_fs::{Bus, CreateParams, UHIDDevice};

pub struct HIDController {
    pub device: UHIDDevice<File>,
}

impl TryFrom<String> for HIDController {
    type Error = io::Error;
    fn try_from(name: String) -> io::Result<Self> {
        let mut rd_data = ArrayVec::new();
        rd_data.try_extend_from_slice(&RDESC).unwrap();
        let params = CreateParams {
            name: ArrayString::from(&name).unwrap(),
            phys: ArrayString::new(),
            uniq: ArrayString::new(),
            bus: Bus::USB,
            vendor: 0x15d9,
            product: 0x0a37,
            version: 0,
            country: 0,
            rd_data,
        };
        Ok(Self {
            device: UHIDDevice::try_new(params)?,
        })
    }
}

impl Drop for HIDController {
    fn drop(&mut self) {
        self.device.destroy().unwrap();
    }
}

// Formulate a 'HID Report Descriptor' to describe the function of your device.
// This tells the kernel how to interpret the HID packets you send to the device.
const RDESC: [u8; 52] = [
    0x05, 0x01, 0x09, 0x02, 0xa1, 0x01, 0x09, 0x01, 0xa1, 0x00, 0x05, 0x09, 0x19, 0x01, 0x29, 0x03,
    0x15, 0x00, 0x25, 0x01, 0x95, 0x03, 0x75, 0x01, 0x81, 0x02, 0x95, 0x01, 0x75, 0x05, 0x81, 0x01,
    0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x09, 0x38, 0x15, 0x80, 0x25, 0x7f, 0x75, 0x08, 0x95, 0x03,
    0x81, 0x06, 0xc0, 0xc0,
];
/*
const RDESC: [u8; 85] = [
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    0x09, 0x02, /* USAGE (Mouse) */
    0xa1, 0x01, /* COLLECTION (Application) */
    0x09, 0x01, /* USAGE (Pointer) */
    0xa1, 0x00, /* COLLECTION (Physical) */
    0x85, 0x01, /* REPORT_ID (1) */
    0x05, 0x09, /* USAGE_PAGE (Button) */
    0x19, 0x01, /* USAGE_MINIMUM (Button 1) */
    0x29, 0x03, /* USAGE_MAXIMUM (Button 3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0x81, 0x02, /* INPUT (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x81, 0x01, /* INPUT (Cnst,Var,Abs) */
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    0x09, 0x30, /* USAGE (X) */
    0x09, 0x31, /* USAGE (Y) */
    0x09, 0x38, /* USAGE (WHEEL) */
    0x15, 0x81, /* LOGICAL_MINIMUM (-127) */
    0x25, 0x7f, /* LOGICAL_MAXIMUM (127) */
    0x75, 0x08, /* REPORT_SIZE (8) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x81, 0x06, /* INPUT (Data,Var,Rel) */
    0xc0, /* END_COLLECTION */
    0xc0, /* END_COLLECTION */
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    0x09, 0x06, /* USAGE (Keyboard) */
    0xa1, 0x01, /* COLLECTION (Application) */
    0x85, 0x02, /* REPORT_ID (2) */
    0x05, 0x08, /* USAGE_PAGE (Led) */
    0x19, 0x01, /* USAGE_MINIMUM (1) */
    0x29, 0x03, /* USAGE_MAXIMUM (3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0x91, 0x02, /* Output (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x91, 0x01, /* Output (Cnst,Var,Abs) */
    0xc0, /* END_COLLECTION */
];
*/
