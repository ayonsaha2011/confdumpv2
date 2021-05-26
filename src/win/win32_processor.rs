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
pub struct Win32_Processor {
    pub SystemName: Option<String>,
    pub ThreadCount: Option<u32>,
    pub AddressWidth: Option<u16>,
    pub DataWidth: Option<u16>,
    pub PNPDeviceID: Option<String>,
    pub DeviceID: Option<String>,
    pub Availability: Option<u16>,
    pub ExtClock: Option<u32>,
    pub NumberOfEnabledCore: Option<u32>,
    pub PowerManagementSupported: Option<bool>,
    pub Stepping: Option<String>,
    pub VMMonitorModeExtensions: Option<bool>,
    pub ErrorCleared: Option<bool>,
    pub Status: Option<String>,
    pub AssetTag: Option<String>,
    pub ErrorDescription: Option<String>,
    pub UniqueId: Option<String>,
    pub Role: Option<String>,
    pub CpuStatus: Option<u16>,
    pub SerialNumber: Option<String>,
    pub NumberOfCores: Option<u32>,
    pub Description: Option<String>,
    pub VirtualizationFirmwareEnabled: Option<bool>,
    pub UpgradeMethod: Option<u16>,
    pub LoadPercentage: Option<u16>,
    pub LastErrorCode: Option<u32>,
    pub L2CacheSpeed: Option<u32>,
    pub Level: Option<u16>,
    pub CreationClassName: Option<String>,
    pub Revision: Option<u16>,
    pub MaxClockSpeed: Option<u32>,
    pub Family: Option<u16>,
    pub SecondLevelAddressTranslationExtensions: Option<bool>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub ProcessorId: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Manufacturer: Option<String>,
    pub L3CacheSize: Option<u32>,
    pub NumberOfLogicalProcessors: Option<u32>,
    pub Characteristics: Option<u32>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub Version: Option<String>,
    pub VoltageCaps: Option<u32>,
    pub L3CacheSpeed: Option<u32>,
    pub ProcessorType: Option<u16>,
    pub Caption: Option<String>,
    pub L2CacheSize: Option<u32>,
    pub StatusInfo: Option<u16>,
    pub PartNumber: Option<String>,
    pub Name: Option<String>,
    pub Architecture: Option<u16>,
    pub OtherFamilyDescription: Option<String>,
    pub CurrentClockSpeed: Option<u32>,
    pub SystemCreationClassName: Option<String>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub SocketDesignation: Option<String>,
    pub CurrentVoltage: Option<u16>,
}
