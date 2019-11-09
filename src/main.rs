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
        let mut any = false;
        blue_controller.poll_values().for_each(|value| {
            let packet = Packet::from(&*value);
            if let Some(last) = last_opt.clone() {
                let delta = if packet.axis == Axis::ZERO || last.axis == Axis::ZERO {
                    Axis::ZERO
                } else {
                    packet.axis - last.axis
                };
                let delta_hor = delta.x as i8 as u8;
                let delta_ver = delta.y as i8 as u8;
                let wheel = if packet.buttons.volume_up {
                    1 as i8
                } else if packet.buttons.volume_down {
                    -1 as i8
                } else {
                    0 as i8
                } as u8;

                let mut data = ArrayVec::new();
                data.try_extend_from_slice(&[packet.buttons.into(), delta_hor, delta_ver, wheel])
                    .unwrap();
                hid_controller.device.send_input(data).unwrap();
            }
            last_opt = Some(packet);
            any = true;
        });
        if !any {
            last_opt = None;
        }
    }
}
