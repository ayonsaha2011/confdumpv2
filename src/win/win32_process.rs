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
pub struct Win32_Process {
    pub CSName: Option<String>,
    pub MaximumWorkingSetSize: Option<u32>,
    pub OtherOperationCount: Option<String>,
    pub ProcessId: Option<u32>,
    pub Caption: Option<String>,
    pub OSName: Option<String>,
    pub QuotaPagedPoolUsage: Option<u32>,
    pub PeakPageFileUsage: Option<u32>,
    pub SessionId: Option<u32>,
    pub ExecutionState: Option<u16>,
    pub QuotaPeakPagedPoolUsage: Option<u32>,
    pub PeakVirtualSize: Option<String>,
    pub Handle: Option<String>,
    pub PrivatePageCount: Option<String>,
    pub PageFileUsage: Option<u32>,
    pub Status: Option<String>,
    pub CommandLine: Option<String>,
    pub UserModeTime: Option<String>,
    pub WriteTransferCount: Option<String>,
    pub QuotaNonPagedPoolUsage: Option<u32>,
    pub KernelModeTime: Option<String>,
    pub VirtualSize: Option<String>,
    pub CSCreationClassName: Option<String>,
    pub OSCreationClassName: Option<String>,
    pub ReadTransferCount: Option<String>,
    pub HandleCount: Option<u32>,
    pub CreationDate: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
    pub MinimumWorkingSetSize: Option<u32>,
    pub OtherTransferCount: Option<String>,
    pub Priority: Option<u32>,
    pub WindowsVersion: Option<String>,
    pub WorkingSetSize: Option<String>,
    pub PeakWorkingSetSize: Option<u32>,
    pub ExecutablePath: Option<String>,
    pub CreationClassName: Option<String>,
    pub ThreadCount: Option<u32>,
    pub ReadOperationCount: Option<String>,
    pub Description: Option<String>,
    pub WriteOperationCount: Option<String>,
    pub PageFaults: Option<u32>,
    pub ParentProcessId: Option<u32>,
    pub QuotaPeakNonPagedPoolUsage: Option<u32>,
    pub TerminationDate: Option<WMIDateTime>,
}
