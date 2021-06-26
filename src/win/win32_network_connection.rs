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
pub struct Win32_NetworkConnection {
    pub Caption: Option<String>,
    pub Description: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
    pub Status: Option<String>,
    pub AccessMask: Option<u32>,
    pub Comment: Option<String>,
    pub ConnectionState: Option<String>,
    pub ConnectionType: Option<String>,
    pub DisplayType: Option<String>,
    pub LocalName: Option<String>,
    pub Persistent: Option<bool>,
    pub ProviderName: Option<String>,
    pub RemoteName: Option<String>,
    pub RemotePath: Option<String>,
    pub ResourceType: Option<String>,
    pub UserName: Option<String>
}
