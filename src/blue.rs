use blurz::{
    BluetoothAdapter, BluetoothDevice, BluetoothGATTCharacteristic, BluetoothGATTService,
    BluetoothSession,
};

const SERVICE_UUID: &str = "4f63756c-7573-2054-6872-65656d6f7465";
const CHAR_WRITE_UUID: &str = "c8c51726-81bc-483b-a052-f7a14ea3d282";
const CHAR_NOTIFY_UUID: &str = "c8c51726-81bc-483b-a052-f7a14ea3d281";

pub struct Controller<'a> {
    pub device: BluetoothDevice<'a>,
    pub writer: BluetoothGATTCharacteristic<'a>,
    pub notify: BluetoothGATTCharacteristic<'a>,
}

impl<'a> Controller<'a> {
    pub fn new_vec(session: &'a BluetoothSession) -> Vec<Controller<'a>> {
        let adapter = BluetoothAdapter::init(session).unwrap();
        let devices = adapter.get_device_list().unwrap();
        devices
            .iter()
            .map(|object_path| BluetoothDevice::new(session, object_path.clone()))
            .filter(|device| device.is_connected().unwrap())
            .filter(|device| {
                device
                    .get_uuids()
                    .unwrap()
                    .iter()
                    .any(|uuid| uuid == SERVICE_UUID)
            })
            .flat_map(|device| {
                let services: Vec<BluetoothGATTService> = device
                    .get_gatt_services()
                    .unwrap()
                    .iter()
                    .map(|object_path| BluetoothGATTService::new(session, object_path.clone()))
                    .filter(|service| service.get_uuid().unwrap() == SERVICE_UUID)
                    .collect();
                let chars: Vec<BluetoothGATTCharacteristic> = services
                    .iter()
                    .flat_map(|service| service.get_gatt_characteristics().unwrap())
                    .map(|path| BluetoothGATTCharacteristic::new(session, path.clone()))
                    .collect();
                let writer = chars
                    .iter()
                    .find(|c| c.get_uuid().unwrap() == CHAR_WRITE_UUID);
                let notify = chars
                    .iter()
                    .find(|c| c.get_uuid().unwrap() == CHAR_NOTIFY_UUID);
                if let (Some(writer), Some(notify)) = (writer, notify) {
                    Some(Controller {
                        device: device,
                        writer: writer.clone(),
                        notify: notify.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
