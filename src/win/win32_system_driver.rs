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
pub struct Win32_SystemDriver {
    pub ExitCode: Option<u32>,
    pub Description: Option<String>,
    pub DisplayName: Option<String>,
    pub AcceptPause: Option<bool>,
    pub PathName: Option<String>,
    pub Started: Option<bool>,
    pub StartName: Option<String>,
    pub ServiceSpecificExitCode: Option<u32>,
    pub State: Option<String>,
    pub StartMode: Option<String>,
    pub Status: Option<String>,
    pub SystemName: Option<String>,
    pub CreationClassName: Option<String>,
    pub TagId: Option<u32>,
    pub AcceptStop: Option<bool>,
    pub Caption: Option<String>,
    pub SystemCreationClassName: Option<String>,
    pub ServiceType: Option<String>,
    pub ErrorControl: Option<String>,
    pub DesktopInteract: Option<bool>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
}
