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

    let mut last_opt: Option<Packet> = None;
    loop {
        blue_controller.poll_values().for_each(|value| {
            let packet = Packet::from(&*value);
            let mut data = ArrayVec::new();
            if packet.axis == Axis::ZERO {
                data.try_extend_from_slice(&[
                    packet.buttons.into(),
                    0,
                    0,
                    0,
                ])
                .unwrap();
                last_opt = None;
            } else {
                if let Some(last) = last_opt.clone() {
                    let delta = packet.axis - last.axis;
                    let delta_hor = delta.x as i8 as u8;
                    let delta_ver = delta.y as i8 as u8;
                    data.try_extend_from_slice(&[
                        packet.buttons.into(),
                        delta_hor,
                        delta_ver,
                        0,
                    ])
                    .unwrap();
                    hid_controller.device.send_input(data).unwrap();
                }
                last_opt = Some(packet);
            }
        });
    }
}
