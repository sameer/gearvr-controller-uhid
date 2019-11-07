use std::error::Error;

use blurz::{
    BluetoothAdapter, BluetoothDevice, BluetoothEvent, BluetoothGATTCharacteristic,
    BluetoothGATTService, BluetoothSession,
};

use crate::decode::*;

const SERVICE_UUID: &str = "4f63756c-7573-2054-6872-65656d6f7465";
const CHAR_WRITE_UUID: &str = "c8c51726-81bc-483b-a052-f7a14ea3d282";
const CHAR_NOTIFY_UUID: &str = "c8c51726-81bc-483b-a052-f7a14ea3d281";

pub struct BlueController<'a> {
    session: &'a BluetoothSession,
    pub device: BluetoothDevice<'a>,
    pub writer: BluetoothGATTCharacteristic<'a>,
    pub notify: BluetoothGATTCharacteristic<'a>,
}

impl<'a> BlueController<'a> {
    pub fn new_vec(session: &'a BluetoothSession) -> Vec<BlueController<'a>> {
        let adapter = BluetoothAdapter::init(session).unwrap();
        let mut devices = adapter.get_device_list().unwrap();
        devices
            .drain(..)
            .map(|object_path| BluetoothDevice::new(session, object_path))
            .filter(|device| device.is_connected().unwrap())
            .filter(|device| {
                device
                    .get_uuids()
                    .unwrap()
                    .iter()
                    .any(|uuid| uuid == SERVICE_UUID)
            })
            .flat_map(|device| {
                let mut services: Vec<BluetoothGATTService> = device
                    .get_gatt_services()
                    .unwrap()
                    .drain(..)
                    .map(|object_path| BluetoothGATTService::new(session, object_path))
                    .filter(|service| service.get_uuid().unwrap() == SERVICE_UUID)
                    .collect();
                let chars: Vec<BluetoothGATTCharacteristic> = services
                    .drain(..)
                    .flat_map(|service| service.get_gatt_characteristics().unwrap())
                    .map(|path| BluetoothGATTCharacteristic::new(session, path))
                    .collect();
                let writer = chars
                    .iter()
                    .find(|c| c.get_uuid().unwrap() == CHAR_WRITE_UUID);
                let notify = chars.iter().find(|c| {
                    c.get_uuid()
                        .map(|uuid| uuid == CHAR_NOTIFY_UUID)
                        .unwrap_or_default()
                });
                if let (Some(writer), Some(notify)) = (writer, notify) {
                    Some(BlueController {
                        session,
                        device,
                        writer: writer.clone(),
                        notify: notify.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn command(&mut self, command: Command) -> Result<(), Box<dyn Error>> {
        self.writer.write_value(command.value().to_vec(), None)
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.command(Command::Sensor)?;
        self.notify.start_notify()
    }

    pub fn poll_values(&mut self) -> impl Iterator<Item = Box<[u8]>> + 'a {
        let id = self.notify.get_id();
        self.session
            .incoming(1000)
            .map(BluetoothEvent::from)
            .filter(Option::is_some)
            .flat_map(move |event| match event.unwrap() {
                BluetoothEvent::Value { object_path, value } => {
                    if object_path == id {
                        Some(value)
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }
}
