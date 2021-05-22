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

#[derive(Serialize, Deserialize, Debug)]
pub struct Win32_OperatingSystem {
    pub EncryptionLevel: Option<i32>,
    pub OSProductSuite: Option<i32>,
    pub OSType: Option<u16>,
    pub ServicePackMajorVersion: Option<i32>,
    pub DataExecutionPrevention_SupportPolicy: Option<i32>,
    pub ServicePackMinorVersion: Option<i32>,
    pub NumberOfUsers: Option<i32>,
    pub ForegroundApplicationBoost: Option<i32>,
    pub OperatingSystemSKU: Option<i32>,
    pub SuiteMask: Option<i32>,
    pub NumberOfProcesses: Option<i32>,
    pub ProductType: Option<i32>,
    pub CurrentTimeZone: Option<i16>,
    pub MaxNumberOfProcesses: Option<i32>,
    pub OSLanguage: Option<i32>,
    pub NumberOfLicensedUsers: Option<u32>,
    pub TotalSwapSpaceSize: Option<u64>,
    pub DataExecutionPrevention_32BitApplications: Option<bool>,
    pub DataExecutionPrevention_Available: Option<bool>,
    pub Primary: Option<bool>,
    pub Distributed: Option<bool>,
    pub PortableOperatingSystem: Option<bool>,
    pub Debug: Option<bool>,
    pub DataExecutionPrevention_Drivers: Option<bool>,
    pub Organization: Option<String>,
    pub CountryCode: Option<String>,
    pub SerialNumber: Option<String>,
    pub BootDevice: Option<String>,
    pub Version: Option<String>,
    pub LocalDateTime: Option<String>,
    pub Name: Option<String>,
    pub SystemDevice: Option<String>,
    pub Locale: Option<String>,
    pub Caption: Option<String>,
    pub Status: Option<String>,
    pub SystemDrive: Option<String>,
    pub CodeSet: Option<String>,
    pub BuildNumber: Option<String>,
    pub TotalVisibleMemorySize: Option<String>,
    pub InstallDate: Option<String>,
    pub FreeVirtualMemory: Option<String>,
    pub RegisteredUser: Option<String>,
    pub WindowsDirectory: Option<String>,
    pub CSCreationClassName: Option<String>,
    pub CreationClassName: Option<String>,
    pub BuildType: Option<String>,
    pub CSName: Option<String>,
    pub TotalVirtualMemorySize: Option<String>,
    pub SizeStoredInPagingFiles: Option<String>,
    pub FreePhysicalMemory: Option<String>,
    pub Description: Option<String>,
    pub FreeSpaceInPagingFiles: Option<String>,
    pub OSArchitecture: Option<String>,
    pub Manufacturer: Option<String>,
    pub MaxProcessMemorySize: Option<String>,
    pub SystemDirectory: Option<String>,
    pub LastBootUpTime: Option<String>,
    pub MUILanguages: Option<Vec<String>>
}
