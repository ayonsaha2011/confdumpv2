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
pub struct Win32_CDROMDrive {
    pub Caption: Option<String>,
    pub Description: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
    pub Status: Option<String>,
    pub Availability: Option<u16>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub CreationClassName: Option<String>,
    pub DeviceID: Option<String>,
    pub ErrorCleared: Option<bool>,
    pub ErrorDescription: Option<String>,
    pub LastErrorCode: Option<u32>,
    pub PNPDeviceID: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub PowerManagementSupported: Option<bool>,
    pub StatusInfo: Option<u16>,
    pub SystemCreationClassName: Option<String>,
    pub SystemName: Option<String>,
    pub Capabilities: Option<Vec<i32>>,
    pub CapabilityDescriptions: Option<Vec<String>>,
    pub CompressionMethod: Option<String>,
    pub DefaultBlockSize: Option<u64>,
    pub ErrorMethodology: Option<String>,
    pub MaxBlockSize: Option<u64>,
    pub MaxMediaSize: Option<u64>,
    pub MinBlockSize: Option<u64>,
    pub NeedsCleaning: Option<bool>,
    pub NumberOfMediaSupported: Option<u32>,
    pub Drive: Option<String>,
    pub DriveIntegrity: Option<bool>,
    pub FileSystemFlags: Option<u16>,
    pub FileSystemFlagsEx: Option<u32>,
    pub Id: Option<String>,
    pub Manufacturer: Option<String>,
    pub MaximumComponentLength: Option<u32>,
    pub MediaLoaded: Option<bool>,
    pub MediaType: Option<String>,
    pub MfrAssignedRevisionLevel: Option<String>,
    pub RevisionLevel: Option<String>,
    pub SCSIBus: Option<u32>,
    pub SCSILogicalUnit: Option<u16>,
    pub SCSIPort: Option<u16>,
    pub SCSITargetId: Option<u16>,
    pub SerialNumber: Option<String>,
    pub Size: Option<u64>,
    pub TransferRate: Option<f64>,
    pub VolumeName: Option<String>,
    pub VolumeSerialNumber: Option<String>
}
