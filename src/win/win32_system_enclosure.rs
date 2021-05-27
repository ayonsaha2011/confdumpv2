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
pub struct Win32_SystemEnclosure {
    pub BreachDescription: Option<String>,
    pub Depth: Option<f32>,
    pub ChassisTypes: Option<Vec<i32>>,
    pub VisibleAlarm: Option<bool>,
    pub AudibleAlarm: Option<bool>,
    pub SKU: Option<String>,
    pub ServiceDescriptions: Option<Vec<String>>,
    pub Status: Option<String>,
    pub OtherIdentifyingInfo: Option<String>,
    pub PoweredOn: Option<bool>,
    pub TypeDescriptions: Option<Vec<String>>,
    pub Weight: Option<f32>,
    pub CreationClassName: Option<String>,
    pub ServicePhilosophy: Option<Vec<i32>>,
    pub NumberOfPowerCords: Option<u16>,
    pub Name: Option<String>,
    pub CurrentRequiredOrProduced: Option<i16>,
    pub Height: Option<f32>,
    pub Removable: Option<bool>,
    pub Caption: Option<String>,
    pub Tag: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Width: Option<f32>,
    pub SerialNumber: Option<String>,
    pub HeatGeneration: Option<u16>,
    pub SMBIOSAssetTag: Option<String>,
    pub CableManagementStrategy: Option<String>,
    pub LockPresent: Option<bool>,
    pub SecurityStatus: Option<u16>,
    pub Version: Option<String>,
    pub HotSwappable: Option<bool>,
    pub Replaceable: Option<bool>,
    pub Manufacturer: Option<String>,
    pub PartNumber: Option<String>,
    pub Description: Option<String>,
    pub Model: Option<String>,
    pub SecurityBreach: Option<u16>,
}
