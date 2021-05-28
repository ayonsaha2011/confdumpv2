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
pub struct Win32_Printer {
    pub Description: Option<String>,
    pub MaxSizeSupported: Option<u32>,
    pub PrinterState: Option<u32>,
    pub CurrentPaperType: Option<String>,
    pub SystemName: Option<String>,
    pub Network: Option<bool>,
    pub ErrorInformation: Option<Vec<String>>,
    pub CurrentNaturalLanguage: Option<String>,
    pub UntilTime: Option<WMIDateTime>,
    pub CurrentLanguage: Option<u16>,
    pub Priority: Option<u32>,
    pub Direct: Option<bool>,
    pub ServerName: Option<String>,
    pub PaperSizesSupported: Option<Vec<i32>>,
    pub Comment: Option<String>,
    pub DeviceID: Option<String>,
    pub Status: Option<String>,
    pub Local: Option<bool>,
    pub PrintJobDataType: Option<String>,
    pub Location: Option<String>,
    pub NaturalLanguagesSupported: Option<Vec<String>>,
    pub DefaultPaperType: Option<String>,
    pub Shared: Option<bool>,
    pub SpoolEnabled: Option<bool>,
    pub JobCountSinceLastReset: Option<u32>,
    pub Hidden: Option<bool>,
    pub DefaultNumberUp: Option<u32>,
    pub DefaultPriority: Option<u32>,
    pub CurrentMimeType: Option<String>,
    pub HorizontalResolution: Option<u32>,
    pub CapabilityDescriptions: Option<Vec<String>>,
    pub PNPDeviceID: Option<String>,
    pub AvailableJobSheets: Option<Vec<String>>,
    pub Name: Option<String>,
    pub RawOnly: Option<bool>,
    pub Capabilities: Option<Vec<i32>>,
    pub DefaultCopies: Option<u32>,
    pub MarkingTechnology: Option<u16>,
    pub TimeOfLastReset: Option<WMIDateTime>,
    pub WorkOffline: Option<bool>,
    pub ExtendedPrinterStatus: Option<u16>,
    pub AveragePagesPerMinute: Option<u32>,
    pub ErrorDescription: Option<String>,
    pub MimeTypesSupported: Option<Vec<String>>,
    pub DefaultCapabilities: Option<Vec<i32>>,
    pub EnableBIDI: Option<bool>,
    pub ExtendedDetectedErrorState: Option<u16>,
    pub LastErrorCode: Option<u32>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub StatusInfo: Option<u16>,
    pub Availability: Option<u16>,
    pub DefaultLanguage: Option<u16>,
    pub PrinterPaperNames: Option<Vec<String>>,
    pub Queued: Option<bool>,
    pub DoCompleteFirst: Option<bool>,
    pub SeparatorFile: Option<String>,
    pub DetectedErrorState: Option<u16>,
    pub CurrentCharSet: Option<String>,
    pub PrinterStatus: Option<u16>,
    pub DriverName: Option<String>,
    pub Published: Option<bool>,
    pub Default: Option<bool>,
    pub CurrentCapabilities: Option<Vec<i32>>,
    pub DefaultMimeType: Option<String>,
    pub EnableDevQueryPrint: Option<bool>,
    pub KeepPrintedJobs: Option<bool>,
    pub PaperTypesAvailable: Option<Vec<String>>,
    pub Attributes: Option<u32>,
    pub CreationClassName: Option<String>,
    pub Parameters: Option<String>,
    pub PowerManagementSupported: Option<bool>,
    pub LanguagesSupported: Option<Vec<i32>>,
    pub PrintProcessor: Option<String>,
    pub VerticalResolution: Option<u32>,
    pub Caption: Option<String>,
    pub CharSetsSupported: Option<Vec<String>>,
    pub InstallDate: Option<WMIDateTime>,
    pub MaxNumberUp: Option<u32>,
    pub MaxCopies: Option<u32>,
    pub ShareName: Option<String>,
    pub PortName: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub StartTime: Option<WMIDateTime>,
    pub SystemCreationClassName: Option<String>,
    pub ErrorCleared: Option<bool>,
    pub ConfigManagerUserConfig: Option<bool>,
}
