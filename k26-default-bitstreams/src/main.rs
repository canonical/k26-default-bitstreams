// This file is part of k26-default-bitstreams, an accompanying application to fpgad (https://github.com/canonical/fpgad)
// Copyright 2025 Canonical Ltd.
// SPDX-License-Identifier: GPL-3.0-only
// k26-default-bitstreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3, as published by the Free Software Foundation.
// k26-default-bitstreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranties of MERCHANTABILITY, SATISFACTORY QUALITY, or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program.  If not, see http://www.gnu.org/licenses/.

mod proxies;

use log::{error, info, trace};
use proxies::control_proxy;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use zbus::Connection;

/// Sends the dbus command to load a bitstream
async fn call_apply_overlay(
    platform_str: &str,
    device_handle: &str,
    dtbo_path: &str,
    firmware_lookup_path: &str,
) -> Result<String, zbus::Error> {
    let connection = Connection::system().await?;
    let proxy = control_proxy::ControlProxy::new(&connection).await?;
    proxy
        .apply_overlay(platform_str, device_handle, dtbo_path, firmware_lookup_path)
        .await
}

async fn call_set_flags(platform_str: &str, device_handle: &str, flags: u32) -> Result<String, zbus::Error> {
    let connection = Connection::system().await?;
    let proxy = control_proxy::ControlProxy::new(&connection).await?;
    proxy
        .set_fpga_flags(platform_str, device_handle, flags)
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    trace!("Attempting to load default bitstream.");
    let snap = env::var("SNAP").expect("SNAP not set");
    let source = PathBuf::from(snap)
        .join("data/k26-starter-kits/k26_starter_kits.dtbo")
        .to_string_lossy()
        .to_string();
    match call_set_flags("xlnx", "fpga0", 0).await{
        Ok(msg) => {
            info!("set_fpga_flags response: {msg}");
        }
        Err(e) => {
            error!("{e}");
            return Err(e.into())
        }
    }
    match call_apply_overlay("xlnx", "fpga0", &source, "").await {
        Ok(msg) => {
            info!("apply_overlay response: {msg}");
            Ok(())
        }
        Err(e) => {
            error!("{e}");
            Err(e.into())
        }
    }
}
