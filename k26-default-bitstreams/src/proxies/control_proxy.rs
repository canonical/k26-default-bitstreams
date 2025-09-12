// This file is part of k26-default-bitstreams, an accompanying application to fpgad (https://github.com/canonical/fpgad)
// Copyright 2025 Canonical Ltd.
// SPDX-License-Identifier: GPL-3.0-only
// k26-default-bitstreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3, as published by the Free Software Foundation.
// k26-default-bitstreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranties of MERCHANTABILITY, SATISFACTORY QUALITY, or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program.  If not, see http://www.gnu.org/licenses/.

use zbus::{Result, proxy};
#[proxy(
    default_service = "com.canonical.fpgad",
    interface = "com.canonical.fpgad.control",
    default_path = "/com/canonical/fpgad/control"
)]
pub trait Control {
    async fn set_fpga_flags(
        &self,
        platform_string: &str,
        device_handle: &str,
        flags: u32,
    ) -> Result<String>;

    async fn write_bitstream_direct(
        &self,
        platform_string: &str,
        device_handle: &str,
        bitstream_path_str: &str,
        firmware_lookup_path: &str,
    ) -> Result<String>;
}
