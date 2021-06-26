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
pub struct Win32_PnPEntity {
    pub ConfigManagerErrorCode: Option<u32>,
    pub Present: Option<bool>,
    pub Caption: Option<String>,
    pub Description: Option<String>,
    pub Name: Option<String>,
    pub PNPDeviceID: Option<String>,
    pub StatusInfo: Option<u16>,
    pub HardwareID: Option<Vec<String>>,
    pub LastErrorCode: Option<u32>,
    pub CreationClassName: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub SystemCreationClassName: Option<String>,
    pub SystemName: Option<String>,
    pub CompatibleID: Option<Vec<String>>,
    pub InstallDate: Option<WMIDateTime>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub Service: Option<String>,
    pub ErrorCleared: Option<bool>,
    pub PowerManagementSupported: Option<bool>,
    pub DeviceID: Option<String>,
    pub ErrorDescription: Option<String>,
    pub Status: Option<String>,
    pub Availability: Option<u16>,
    pub ClassGuid: Option<String>,
    pub PNPClass: Option<String>,
    pub Manufacturer: Option<String>,
}
