use std::convert::TryFrom;

use blurz::BluetoothSession;
use arrayvec::ArrayVec;

mod blue;
mod decode;
mod hid;

use blue::*;
use decode::*;
use hid::*;

fn main() {
    let session = BluetoothSession::create_session(None).unwrap();
    let blue_controllers = BlueController::new_vec(&session);
    let blue_controller = blue_controllers.iter().nth(0).unwrap();
    blue_controller
        .writer
        .write_value(Command::Sensor.value().to_vec(), None)
        .unwrap();
    // let (fd, count) = notify.acquire_notify().unwrap();
    blue_controller.notify.start_notify().unwrap();

    let mut hid_controller = HIDController::try_from(blue_controller.device.get_name().unwrap()).unwrap();

    let mut last_axis: Option<Axis> = None;
    loop {
        let value = blue_controller.notify.get_value().unwrap();
        let packet = Packet::from(value.as_slice());
        if packet.axis == Axis::default() {
            last_axis = None;
        } else if let Some(last) = last_axis {
            let delta = packet.axis - last;
            let delta_hor = delta.x as u8;
            let delta_ver = delta.y as u8;
            let mut data = ArrayVec::new();
            data.try_extend_from_slice(&[1, 0, delta_hor, delta_ver, 0]).unwrap();
            
            hid_controller.device.send_input(data).unwrap();
            last_axis = Some(packet.axis);
        } else {
            last_axis = Some(packet.axis);
        }
        std::thread::sleep_ms(10);
    }
}
