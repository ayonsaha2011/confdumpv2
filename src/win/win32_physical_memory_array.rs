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
pub struct win32_PhysicalMemoryArray {
    pub Use: Option<u16>,
    pub Model: Option<String>,
    pub MemoryErrorCorrection: Option<u16>,
    pub Width: Option<f32>,
    pub Caption: Option<String>,
    pub Location: Option<u16>,
    pub Name: Option<String>,
    pub SerialNumber: Option<String>,
    pub Height: Option<f32>,
    pub MaxCapacity: Option<u32>,
    pub CreationClassName: Option<String>,
    pub Description: Option<String>,
    pub Depth: Option<f32>,
    pub Version: Option<String>,
    pub PoweredOn: Option<bool>,
    pub MemoryDevices: Option<u16>,
    pub InstallDate: Option<WMIDateTime>,
    pub Tag: Option<String>,
    pub PartNumber: Option<String>,
    pub OtherIdentifyingInfo: Option<String>,
    pub Removable: Option<bool>,
    pub HotSwappable: Option<bool>,
    pub Weight: Option<f32>,
    pub SKU: Option<String>,
    pub Replaceable: Option<bool>,
    pub Manufacturer: Option<String>,
    pub MaxCapacityEx: Option<String>,
    pub Status: Option<String>,
}
