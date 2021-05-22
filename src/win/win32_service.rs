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
pub struct Win32_Service {
    pub AcceptPause:  Option<bool>,
    pub AcceptStop: Option<bool>,
    pub DelayedAutoStart: Option<bool>,
    pub DesktopInteract: Option<bool>,
    pub Started: Option<bool>,
    pub CheckPoint: Option<i32>,
    pub TagId: Option<i32>,
    pub ExitCode: Option<i32>,
    pub ProcessId: Option<i32>,
    pub ServiceSpecificExitCode: Option<i32>,
    pub WaitHint: Option<i32>,
    pub Caption: Option<String>,
    pub CreationClassName: Option<String>,
    pub Description: Option<String>,
    pub DisplayName: Option<String>,
    pub ErrorControl: Option<String>,
    pub Name: Option<String>,
    pub PathName: Option<String>,
    pub ServiceType: Option<String>,
    pub StartMode: Option<String>,
    pub StartName: Option<String>,
    pub State: Option<String>,
    pub Status: Option<String>,
    pub SystemCreationClassName: Option<String>,
    pub SystemName: Option<String>,
    pub InstallDate: Option<String>
}