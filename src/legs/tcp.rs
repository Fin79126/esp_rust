use core::sync::atomic::Ordering;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::{ATTACH_SERVO, DETACH_SERVO, HEARTBEAT};
use anyhow::Result;
use common::{ESPStatus, MainCommand};

pub fn tcp_task() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6001")?;

    for stream in listener.incoming() {
        log::info!("TCP connection established");
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                log::error!("accept error: {:?}", e);
                continue;
            }
        };
        let mut buf = Vec::with_capacity(256);
        let mut len_buf = [0u8; 4];
        log::info!("Waiting for TCP commands...");
        loop {
            buf.clear();

            if let Err(e) = stream.read_exact(&mut len_buf) {
                log::warn!("TCP切断検出 (len): {:?}", e);
                DETACH_SERVO.store(true, Ordering::Release);
                break; // ← 接続ループを抜ける
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            // 2. 本体読む
            buf.resize(len, 0);
            if let Err(e) = stream.read_exact(&mut buf) {
                log::warn!("TCP切断検出 (body): {:?}", e);
                DETACH_SERVO.store(true, Ordering::Release);
                break; // ← 接続ループを抜ける
            }

            // --- deserialize ---
            let cmd = match postcard::from_bytes(&buf) {
                Ok(cmd) => cmd,
                Err(e) => {
                    log::error!("deserialize error: {:?}", e);
                    continue; // 切断ではないので継続
                }
            };
            match cmd {
                MainCommand::Setup => {
                    log::info!("Setup command");
                }
                MainCommand::GetStatus => {
                    log::info!("Get status command");
                    let status = ESPStatus {
                        heartbeat: HEARTBEAT.load(Ordering::Relaxed),
                    };
                    let bytes = postcard::to_allocvec(&status)?;
                    let len = (bytes.len() as u32).to_be_bytes();

                    stream.write_all(&len)?;
                    stream.write_all(&bytes)?;
                    stream.flush()?;
                }
                MainCommand::AttachServo => {
                    log::info!("Attach servo command");
                    ATTACH_SERVO.store(true, Ordering::Release);
                }
                MainCommand::DetachServo => {
                    log::info!("Detach servo command");
                    DETACH_SERVO.store(true, Ordering::Release);
                }
            }
        }
    }
    Ok(())
}
