use core::sync::atomic::Ordering;
use esp_idf_svc::hal::delay::FreeRtos;
use std::net::UdpSocket;

use crate::unit::LegsUnit;
use crate::{ATTACH_SERVO, DETACH_SERVO, HEARTBEAT};
use anyhow::Result;
use common::{now_ms, LegsCommand};

pub fn udp_task(legs_unit: &mut LegsUnit) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5001")?;
    socket.set_nonblocking(true)?;

    let mut buf = [0u8; 128];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                if let Ok(cmd) = postcard::from_bytes::<LegsCommand>(&buf[..size]) {
                    // サーボ制御（ここに実装）
                    if let Err(e) = legs_unit.set_units(&cmd) {
                        log::error!("servo error: {:?}", e);
                    }
                    log::info!("Servo: {:?}, BLDC: {:?}", cmd.servo, cmd.bldc);

                    // heartbeat更新
                    HEARTBEAT.store(now_ms(), Ordering::Relaxed);
                }
            }
            Err(_) => {}
        }

        if ATTACH_SERVO.load(Ordering::Acquire) {
            legs_unit.attach_units();
            ATTACH_SERVO.store(false, Ordering::Release);
        }

        if DETACH_SERVO.load(Ordering::Acquire) {
            legs_unit.detach_units();
            DETACH_SERVO.store(false, Ordering::Release);
        }

        FreeRtos::delay_ms(10);
    }
}
