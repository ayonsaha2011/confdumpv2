/*
 * Confdump-Agent - Dump static and runtime system configuration
 * Copyright (C) 2009-2012  Straton IT, SAS
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt;
use serde::{Serialize, Deserialize};
use wmi::WMIDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Win32_LogicalDisk {
    pub Name: Option<String>,
    pub PowerManagementSupported: Option<bool>,
    pub ProviderName: Option<String>,
    pub SupportsDiskQuotas: Option<bool>,
    pub InstallDate: Option<WMIDateTime>,
    pub SystemCreationClassName: Option<String>,
    pub Size: Option<String>,
    pub MaximumComponentLength: Option<u32>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub Caption: Option<String>,
    pub VolumeName: Option<String>,
    pub Compressed: Option<bool>,
    pub CreationClassName: Option<String>,
    pub StatusInfo: Option<u16>,
    pub Purpose: Option<String>,
    pub QuotasRebuilding: Option<bool>,
    pub ErrorCleared: Option<bool>,
    pub LastErrorCode: Option<u32>,
    pub DeviceID: Option<String>,
    pub QuotasIncomplete: Option<bool>,
    pub VolumeDirty: Option<bool>,
    pub FileSystem: Option<String>,
    pub MediaType: Option<u32>,
    pub VolumeSerialNumber: Option<String>,
    pub NumberOfBlocks: Option<u64>,
    pub Access: Option<u16>,
    pub SupportsFileBasedCompression: Option<bool>,
    pub Description: Option<String>,
    pub ErrorMethodology: Option<String>,
    pub QuotasDisabled: Option<bool>,
    pub DriveType: Option<u32>,
    pub SystemName: Option<String>,
    pub BlockSize: Option<u64>,
    pub ErrorDescription: Option<String>,
    pub PNPDeviceID: Option<String>,
    pub FreeSpace: Option<String>,
    pub Status: Option<String>,
    pub Availability: Option<u16>,
}
