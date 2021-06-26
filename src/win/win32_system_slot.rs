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
pub struct Win32_SystemSlot {
    pub SupportsHotPlug: Option<bool>,
    pub ConnectorPinout: Option<String>,
    pub SegmentGroupNumber: Option<u32>,
    pub VppMixedVoltageSupport: Option<Vec<i32>>,
    pub ConnectorType: Option<Vec<i32>>,
    pub PoweredOn: Option<bool>,
    pub Tag: Option<String>,
    pub Description: Option<String>,
    pub Shared: Option<bool>,
    pub VccMixedVoltageSupport: Option<Vec<i32>>,
    pub FunctionNumber: Option<u32>,
    pub InstallDate: Option<WMIDateTime>,
    pub MaxDataWidth: Option<u16>,
    pub BusNumber: Option<u32>,
    pub Model: Option<String>,
    pub SpecialPurpose: Option<bool>,
    pub Manufacturer: Option<String>,
    pub CurrentUsage: Option<u16>,
    pub ThermalRating: Option<u32>,
    pub Name: Option<String>,
    pub Status: Option<String>,
    pub Version: Option<String>,
    pub OtherIdentifyingInfo: Option<String>,
    pub PMESignal: Option<bool>,
    pub SKU: Option<String>,
    pub PurposeDescription: Option<String>,
    pub CreationClassName: Option<String>,
    pub DeviceNumber: Option<u32>,
    pub Number: Option<u16>,
    pub Caption: Option<String>,
    pub PartNumber: Option<String>,
    pub HeightAllowed: Option<f32>,
    pub SlotDesignation: Option<String>,
    pub LengthAllowed: Option<f32>,
    pub SerialNumber: Option<String>,
}
