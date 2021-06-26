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
pub struct Win32_DiskPartition {
    pub SystemCreationClassName: Option<String>,
    pub BootPartition: Option<bool>,
    pub ErrorCleared: Option<bool>,
    pub ErrorMethodology: Option<String>,
    pub PowerManagementSupported: Option<bool>,
    pub Index: Option<u32>,
    pub Purpose: Option<String>,
    pub NumberOfBlocks: Option<String>,
    pub Description: Option<String>,
    pub HiddenSectors: Option<u32>,
    pub PNPDeviceID: Option<String>,
    pub RewritePartition: Option<bool>,
    pub Status: Option<String>,
    pub DiskIndex: Option<u32>,
    pub Access: Option<u16>,
    pub InstallDate: Option<WMIDateTime>,
    pub Size: Option<String>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub PrimaryPartition: Option<bool>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub BlockSize: Option<String>,
    pub SystemName: Option<String>,
    pub Type: Option<String>,
    pub StatusInfo: Option<u16>,
    pub Availability: Option<u16>,
    pub Caption: Option<String>,
    pub ErrorDescription: Option<String>,
    pub LastErrorCode: Option<u32>,
    pub StartingOffset: Option<String>,
    pub Name: Option<String>,
    pub DeviceID: Option<String>,
    pub CreationClassName: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub Bootable: Option<bool>,
}
