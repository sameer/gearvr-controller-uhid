use std::convert::TryFrom;

use arrayvec::ArrayVec;
use blurz::BluetoothSession;

mod blue;
mod decode;
mod hid;

use blue::*;
use decode::*;
use hid::*;

fn main() {
    let session = BluetoothSession::create_session(None).unwrap();
    let mut blue_controller = BlueController::new_vec(&session).drain(..).nth(0).unwrap();

    blue_controller.start().unwrap();

    let mut hid_controller =
        HIDController::try_from(blue_controller.device.get_name().unwrap()).unwrap();

    let mut last_axis: Option<Axis> = None;
    loop {
        blue_controller.poll_values().for_each(|value| {
            let packet = Packet::from(&*value);
            if packet.axis == Axis::ZERO {
                last_axis = None;
            } else if let Some(last) = last_axis {
                let delta = packet.axis - last;
                let delta_hor = delta.x as u8;
                let delta_ver = delta.y as u8;
                let mut data = ArrayVec::new();
                data.try_extend_from_slice(&[1, 0, delta_hor, delta_ver, 0])
                    .unwrap();

                hid_controller.device.send_input(data).unwrap();
                last_axis = Some(packet.axis);
            } else {
                last_axis = Some(packet.axis);
            }
        });
    }
}
