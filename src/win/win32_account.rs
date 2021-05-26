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
pub struct Win32_Account {
    pub Domain: Option<String>,
    pub Status: Option<String>,
    pub Name: Option<String>,
    pub FullName: Option<String>,
    pub PasswordRequired: Option<bool>,
    pub SID: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub LocalAccount: Option<bool>,
    pub PasswordChangeable: Option<bool>,
    pub PasswordExpires: Option<bool>,
    pub Description: Option<String>,
    pub AccountType: Option<i64>,
    pub Caption: Option<String>,
    pub Disabled: Option<bool>,
    pub Lockout: Option<bool>,
    pub SIDType: Option<u8>,
}
