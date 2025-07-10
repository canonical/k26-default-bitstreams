// This file is part of k26-default-bitstreams, an accompanying application to fpgad (https://github.com/canonical/fpgad)
// Copyright 2025 Canonical Ltd.
// SPDX-License-Identifier: GPL-3.0-only
// k26-default-bitstreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3, as published by the Free Software Foundation.
// k26-default-bitstreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranties of MERCHANTABILITY, SATISFACTORY QUALITY, or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program.  If not, see http://www.gnu.org/licenses/.

use zbus::{Result, proxy};
#[proxy(
    default_service = "com.canonical.fpgad",
    interface = "com.canonical.fpgad.status",
    default_path = "/com/canonical/fpgad/status"
)]
pub trait Status {
    async fn get_fpga_state(&self, platform_string: &str, device_handle: &str) -> Result<String>;
    async fn get_fpga_flags(&self, platform_string: &str, device_handle: &str) -> Result<String>;
    async fn get_overlay_status(
        &self,
        platform_compat_str: &str,
        overlay_handle: &str,
    ) -> Result<String>;
    async fn get_overlays(&self) -> Result<String>;
    async fn get_platform_type(&self, device_handle: &str) -> Result<String>;
    async fn get_platform_types(&self) -> Result<String>;
    async fn get_platform_name(&self, _device_handle: &str) -> Result<String>;
}
