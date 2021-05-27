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
pub struct Win32_ComputerSystemProduct {
    pub Description: Option<String>,
    pub ElementName: Option<String>,
    pub Version: Option<String>,
    pub SKUNumber: Option<String>,
    pub Caption: Option<String>,
    pub Name: Option<String>,
    pub UUID: Option<String>,
    pub Vendor: Option<String>,
    pub WarrantyDuration: Option<u32>,
    pub InstanceID: Option<String>,
    pub IdentifyingNumber: Option<String>,
    pub WarrantyStartDate: Option<WMIDateTime>,
}
