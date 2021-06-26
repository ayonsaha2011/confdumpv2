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
pub struct Win32_PhysicalMemory {
    pub MinVoltage: Option<u32>,
    pub Speed: Option<u32>,
    pub ConfiguredVoltage: Option<u32>,
    pub Description: Option<String>,
    pub Model: Option<String>,
    pub SMBIOSMemoryType: Option<u32>,
    pub PartNumber: Option<String>,
    pub TotalWidth: Option<u16>,
    pub Name: Option<String>,
    pub HotSwappable: Option<bool>,
    pub ConfiguredClockSpeed: Option<u32>,
    pub Removable: Option<bool>,
    pub Status: Option<String>,
    pub BankLabel: Option<String>,
    pub InterleavePosition: Option<u32>,
    pub Manufacturer: Option<String>,
    pub MemoryType: Option<u16>,
    pub Attributes: Option<u32>,
    pub Capacity: Option<String>,
    pub DataWidth: Option<u16>,
    pub FormFactor: Option<u16>,
    pub SKU: Option<String>,
    pub Caption: Option<String>,
    pub MaxVoltage: Option<u32>,
    pub InterleaveDataDepth: Option<u16>,
    pub CreationClassName: Option<String>,
    pub PoweredOn: Option<bool>,
    pub DeviceLocator: Option<String>,
    pub TypeDetail: Option<u16>,
    pub OtherIdentifyingInfo: Option<String>,
    pub Version: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub SerialNumber: Option<String>,
    pub Replaceable: Option<bool>,
    pub PositionInRow: Option<u32>,
    pub Tag: Option<String>,
}
