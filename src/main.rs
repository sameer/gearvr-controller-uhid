use blurz::BluetoothSession;

mod blue;
mod decode;
mod hid;

use blue::*;
use decode::*;
use hid::*;

fn main() {
    let session = BluetoothSession::create_session(None).unwrap();
    let controllers = Controller::new_vec(&session);
    let controller = controllers.iter().nth(0).unwrap();
    controller
        .writer
        .write_value(Command::Off.value().to_vec(), None)
        .unwrap();
    // let (fd, count) = notify.acquire_notify().unwrap();
    controller.notify.start_notify().unwrap();

    let params = params(controller.device.get_name().unwrap());
    let mut uhid = tokio_linux_uhid::UHIDDevice::create(params, None).unwrap();

    let mut last_axis: Option<Axis> = None;
    loop {
        let value = controller.notify.get_value().unwrap();
        let packet = Packet::from(value.as_slice());
        if packet.axis == Axis::default() {
            last_axis = None;
        } else if let Some(last) = last_axis {
            let delta = packet.axis - last;
            let delta_hor = delta.x as u8;
            let delta_ver = delta.y as u8;
            let data = [1, 0, delta_hor, delta_ver, 0];
            uhid.send_input(&data).unwrap();
            last_axis = Some(packet.axis);
        } else {
            last_axis = Some(packet.axis);
        }
        std::thread::sleep_ms(10);
    }
}
