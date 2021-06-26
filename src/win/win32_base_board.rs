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
pub struct Win32_BaseBoard {
    pub CreationClassName: Option<String>,
    pub HotSwappable: Option<bool>,
    pub Product: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Status: Option<String>,
    pub Name: Option<String>,
    pub HostingBoard: Option<bool>,
    pub Height: Option<f32>,
    pub SlotLayout: Option<String>,
    pub Depth: Option<f32>,
    pub Version: Option<String>,
    pub ConfigOptions: Option<Vec<String>>,
    pub SerialNumber: Option<String>,
    pub RequiresDaughterBoard: Option<bool>,
    pub Replaceable: Option<bool>,
    pub RequirementsDescription: Option<String>,
    pub SKU: Option<String>,
    pub PoweredOn: Option<bool>,
    pub SpecialRequirements: Option<bool>,
    pub Width: Option<f32>,
    pub OtherIdentifyingInfo: Option<String>,
    pub Weight: Option<f32>,
    pub Removable: Option<bool>,
    pub Caption: Option<String>,
    pub Tag: Option<String>,
    pub Manufacturer: Option<String>,
    pub Model: Option<String>,
    pub PartNumber: Option<String>,
    pub Description: Option<String>,
}
