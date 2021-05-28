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
pub struct Win32_OnBoardDevice {
    pub Caption: Option<String>,
    pub Description: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
    pub Status: Option<String>,
    pub CreationClassName: Option<String>,
    pub Manufacturer: Option<String>,
    pub Model: Option<String>,
    pub OtherIdentifyingInfo: Option<String>,
    pub PartNumber: Option<String>,
    pub PoweredOn: Option<bool>,
    pub SerialNumber: Option<String>,
    pub SKU: Option<String>,
    pub Tag: Option<String>,
    pub Version: Option<String>,
    pub HotSwappable: Option<bool>,
    pub Removable: Option<bool>,
    pub Replaceable: Option<bool>,
    pub DeviceType: Option<u16>,
    pub Enabled: Option<bool>
}
