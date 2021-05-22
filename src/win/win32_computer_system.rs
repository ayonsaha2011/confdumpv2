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
pub struct Win32_ComputerSystem {
    pub PCSystemTypeEx: Option<u16>,
    pub FrontPanelResetStatus: Option<u16>,
    pub PowerSupplyState: Option<u16>,
    pub ChassisBootupState: Option<u16>,
    pub ResetCapability: Option<u16>,
    pub CurrentTimeZone: Option<i16>,
    pub PowerState: Option<u16>,
    pub WakeUpType: Option<u16>,
    pub ResetCount: Option<i16>,
    pub DomainRole: Option<u16>,
    pub KeyboardPasswordStatus: Option<u16>,
    pub ThermalState: Option<u16>,
    pub AdminPasswordStatus: Option<u16>,
    pub BootOptionOnLimit: Option<u16>,
    pub BootOptionOnWatchDog: Option<u16>,
    pub ResetLimit: Option<i16>,
    pub NumberOfProcessors: Option<u32>,
    pub PowerOnPasswordStatus: Option<u16>,
    pub PCSystemType: Option<u16>,
    pub NumberOfLogicalProcessors: Option<u32>,
    pub AutomaticResetCapability: Option<bool>,
    pub BootROMSupported: Option<bool>,
    pub HypervisorPresent: Option<bool>,
    pub AutomaticManagedPagefile: Option<bool>,
    pub InfraredSupported: Option<bool>,
    pub NetworkServerModeEnabled: Option<bool>,
    pub PartOfDomain: Option<bool>,
    pub EnableDaylightSavingsTime: Option<bool>,
    pub AutomaticResetBootOption: Option<bool>,
    pub Status: Option<String>,
    pub DNSHostName: Option<String>,
    pub PauseAfterReset: Option<String>,
    pub Name: Option<String>,
    pub Description: Option<String>,
    pub Model: Option<String>,
    pub Manufacturer: Option<String>,
    pub Caption: Option<String>,
    pub BootupState: Option<String>,
    pub Workgroup: Option<String>,
    pub CreationClassName: Option<String>,
    pub SystemType: Option<String>,
    pub TotalPhysicalMemory: Option<String>,
    pub UserName: Option<String>,
    pub Domain: Option<String>,
    pub SystemFamily: Option<String>,
    pub SystemSKUNumber: Option<String>,
    pub PrimaryOwnerName: Option<String>,
    pub BootStatus: Option<Vec<i32>>,
    pub OEMLogoBitmap: Option<Vec<i32>>,
    pub Roles: Option<Vec<String>>,
    pub OEMStringArray: Option<Vec<String>>
}
