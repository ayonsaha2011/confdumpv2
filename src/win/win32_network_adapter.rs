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
pub struct Win32_NetworkAdapter {
    pub CreationClassName: Option<String>,
    pub PNPDeviceID: Option<String>,
    pub NetworkAddresses: Option<Vec<String>>,
    pub Availability: Option<u16>,
    pub Status: Option<String>,
    pub Caption: Option<String>,
    pub Manufacturer: Option<String>,
    pub NetConnectionID: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub ProductName: Option<String>,
    pub MACAddress: Option<String>,
    pub ErrorCleared: Option<bool>,
    pub SystemCreationClassName: Option<String>,
    pub PowerManagementSupported: Option<bool>,
    pub MaxNumberControlled: Option<u32>,
    pub SystemName: Option<String>,
    pub Description: Option<String>,
    pub Speed: Option<String>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub GUID: Option<String>,
    pub Index: Option<u32>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub StatusInfo: Option<u16>,
    pub TimeOfLastReset: Option<String>,
    pub AdapterType: Option<String>,
    pub MaxSpeed: Option<String>,
    pub Installed: Option<bool>,
    pub InterfaceIndex: Option<u32>,
    pub NetConnectionStatus: Option<u16>,
    pub AdapterTypeId: Option<u16>,
    pub ErrorDescription: Option<String>,
    pub LastErrorCode: Option<u32>,
    pub AutoSense: Option<bool>,
    pub Name: Option<String>,
    pub DeviceID: Option<String>,
    pub NetEnabled: Option<bool>,
    pub PhysicalAdapter: Option<bool>,
    pub ServiceName: Option<String>,
    pub PermanentAddress: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
}
