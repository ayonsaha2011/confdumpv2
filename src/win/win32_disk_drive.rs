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
pub struct Win32_DiskDrive {
    pub Model: Option<String>,
    pub StatusInfo: Option<u16>,
    pub TotalSectors: Option<String>,
    pub TotalTracks: Option<String>,
    pub BytesPerSector: Option<u32>,
    pub SCSILogicalUnit: Option<u16>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub DeviceID: Option<String>,
    pub Caption: Option<String>,
    pub TotalHeads: Option<u32>,
    pub CapabilityDescriptions: Option<Vec<String>>,
    pub ErrorMethodology: Option<String>,
    pub CreationClassName: Option<String>,
    pub CompressionMethod: Option<String>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub FirmwareRevision: Option<String>,
    pub InterfaceType: Option<String>,
    pub LastErrorCode: Option<u32>,
    pub MediaType: Option<String>,
    pub NumberOfMediaSupported: Option<u32>,
    pub PNPDeviceID: Option<String>,
    pub SectorsPerTrack: Option<u32>,
    pub ErrorDescription: Option<String>,
    pub SerialNumber: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub ErrorCleared: Option<bool>,
    pub SCSIBus: Option<u32>,
    pub Capabilities: Option<Vec<i32>>,
    pub Size: Option<String>,
    pub TotalCylinders: Option<String>,
    pub TracksPerCylinder: Option<u32>,
    pub Partitions: Option<u32>,
    pub Signature: Option<u32>,
    pub NeedsCleaning: Option<bool>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub SystemName: Option<String>,
    pub MaxMediaSize: Option<u64>,
    pub MediaLoaded: Option<bool>,
    pub Status: Option<String>,
    pub MinBlockSize: Option<u64>,
    pub MaxBlockSize: Option<u64>,
    pub SystemCreationClassName: Option<String>,
    pub Name: Option<String>,
    pub PowerManagementSupported: Option<bool>,
    pub Manufacturer: Option<String>,
    pub SCSIPort: Option<u16>,
    pub DefaultBlockSize: Option<u64>,
    pub Description: Option<String>,
    pub Availability: Option<u16>,
    pub Index: Option<u32>,
    pub SCSITargetId: Option<u16>,
}
