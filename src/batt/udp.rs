use core::sync::atomic::Ordering;
use esp_idf_svc::hal::delay::FreeRtos;
use std::net::UdpSocket;

use crate::servo::BattServo;
use crate::{ATTACH_SERVO, DETACH_SERVO, HEARTBEAT};
use anyhow::Result;
use common::{now_ms, BattCommand};

pub fn udp_task(batt_servo: &mut BattServo) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5001")?;
    socket.set_nonblocking(true)?;

    let mut buf = [0u8; 128];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                if let Ok(cmd) = postcard::from_bytes::<BattCommand>(&buf[..size]) {
                    // サーボ制御（ここに実装）
                    if let Err(e) = batt_servo.set_servos_angle(&cmd) {
                        log::error!("servo error: {:?}", e);
                    }
                    log::info!("Servo: {:?}", cmd.servo);

                    // heartbeat更新
                    HEARTBEAT.store(now_ms(), Ordering::Relaxed);
                }
            }
            Err(_) => {}
        }

        if ATTACH_SERVO.load(Ordering::Acquire) {
            batt_servo.attach_servos();
            ATTACH_SERVO.store(false, Ordering::Release);
        }

        if DETACH_SERVO.load(Ordering::Acquire) {
            batt_servo.detach_servos();
            DETACH_SERVO.store(false, Ordering::Release);
        }

        FreeRtos::delay_ms(10);
    }
}
