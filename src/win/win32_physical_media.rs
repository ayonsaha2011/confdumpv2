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
pub struct Win32_PhysicalMedia {
    pub InstallDate: Option<WMIDateTime>,
    pub OtherIdentifyingInfo: Option<String>,
    pub SerialNumber: Option<String>,
    pub Caption: Option<String>,
    pub MediaDescription: Option<String>,
    pub Name: Option<String>,
    pub PoweredOn: Option<bool>,
    pub Removable: Option<bool>,
    pub WriteProtectOn: Option<bool>,
    pub PartNumber: Option<String>,
    pub Replaceable: Option<bool>,
    pub CreationClassName: Option<String>,
    pub Description: Option<String>,
    pub HotSwappable: Option<bool>,
    pub MediaType: Option<u16>,
    pub SKU: Option<String>,
    pub CleanerMedia: Option<bool>,
    pub Status: Option<String>,
    pub Capacity: Option<u64>,
    pub Manufacturer: Option<String>,
    pub Model: Option<String>,
    pub Tag: Option<String>,
    pub Version: Option<String>,
}
