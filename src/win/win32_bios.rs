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
pub struct Win32_BIOS {
    pub SoftwareElementState: Option<u16>,
    pub Description: Option<String>,
    pub TargetOperatingSystem: Option<u16>,
    pub EmbeddedControllerMajorVersion: Option<u8>,
    pub PrimaryBIOS: Option<bool>,
    pub InstallableLanguages: Option<u16>,
    pub EmbeddedControllerMinorVersion: Option<u8>,
    pub Status: Option<String>,
    pub SMBIOSBIOSVersion: Option<String>,
    pub BuildNumber: Option<String>,
    pub LanguageEdition: Option<String>,
    pub Manufacturer: Option<String>,
    pub SystemBiosMinorVersion: Option<u8>,
    pub BIOSVersion: Option<Vec<String>>,
    pub IdentificationCode: Option<String>,
    pub SMBIOSPresent: Option<bool>,
    pub SystemBiosMajorVersion: Option<u8>,
    pub BiosCharacteristics: Option<Vec<i32>>,
    pub SMBIOSMajorVersion: Option<u16>,
    pub SMBIOSMinorVersion: Option<u16>,
    pub ReleaseDate: Option<String>,
    pub OtherTargetOS: Option<String>,
    pub Name: Option<String>,
    pub Caption: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub SoftwareElementID: Option<String>,
    pub SerialNumber: Option<String>,
    pub CurrentLanguage: Option<String>,
    pub Version: Option<String>,
    pub ListOfLanguages: Option<Vec<String>>,
    pub CodeSet: Option<String>,
}
