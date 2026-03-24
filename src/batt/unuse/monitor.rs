use core::sync::atomic::{AtomicU32, Ordering};
use esp_idf_svc::hal::delay::FreeRtos;
use std::sync::Arc;
use std::thread;

use crate::udp::udp_task;
use common::now_ms;

pub fn monitor_task(heartbeat: Arc<AtomicU32>) {
    loop {
        let hb = heartbeat.load(Ordering::Relaxed);
        let now = now_ms();

        if now - hb > 3000 {
            println!("UDP task dead → restarting");

            let hb_clone = heartbeat.clone();
            thread::spawn(move || {
                let _ = udp_task(hb_clone);
            });

            FreeRtos::delay_ms(1000);
        }

        FreeRtos::delay_ms(500);
    }
}
