// This file is part of k26-default-bitstreams, an accompanying application to fpgad (https://github.com/canonical/fpgad)
// Copyright 2025 Canonical Ltd.
// SPDX-License-Identifier: GPL-3.0-only
// k26-default-bitstreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3, as published by the Free Software Foundation.
// k26-default-bitstreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranties of MERCHANTABILITY, SATISFACTORY QUALITY, or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program.  If not, see http://www.gnu.org/licenses/.

mod proxies;

use crate::proxies::control_proxy;
use log::{error, info, trace};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use tokio::time::{Duration, Instant, sleep};
use zbus::Connection;


/// Sends the dbus command to load a bitstream
async fn call_load_bitstream(
    platform_str: &str,
    device_handle: &str,
    file_path: &str,
    firmware_lookup_path: &str,
) -> Result<String, zbus::Error> {
    let connection = Connection::system().await?;
    let proxy = control_proxy::ControlProxy::new(&connection).await?;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match proxy
            .write_bitstream_direct(platform_str, device_handle, file_path, firmware_lookup_path)
            .await {
            Ok(str) => {
                info!("Connected to D-Bus Control interface");
                return Ok(format!("{}",str))
            }
            Err(e) => {
                if Instant::now() >= deadline {
                    error!("Timed out after 30s: {e}");
                    return Err(zbus::Error::InterfaceNotFound);
                } else {
                    trace!("Retrying after error: {e}");
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    trace!("Attempting to load default bitstream.");

    let snap = env::var("SNAP").expect("SNAP not set");
    let source = PathBuf::from(snap)
        .join("data/k26-starter-kits/k26_starter_kits.bit.bin")
        .to_string_lossy()
        .to_string();
    match call_load_bitstream("xlnx", "fpga0", &source, "").await {
        Ok(msg) => {
            info!("{msg}")
        }
        Err(e) => {
            error!("{e}")
        }
    }

    Ok(())
}
