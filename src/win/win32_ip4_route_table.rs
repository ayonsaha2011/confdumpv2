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
pub struct Win32_IP4RouteTable {
    pub Protocol: Option<u32>,
    pub Caption: Option<String>,
    pub Name: Option<String>,
    pub Description: Option<String>,
    pub Information: Option<String>,
    pub Metric2: Option<i32>,
    pub InterfaceIndex: Option<i32>,
    pub Metric5: Option<i32>,
    pub NextHop: Option<String>,
    pub Metric4: Option<i32>,
    pub Destination: Option<String>,
    pub Type: Option<u32>,
    pub InstallDate: Option<WMIDateTime>,
    pub Metric3: Option<i32>,
    pub Metric1: Option<i32>,
    pub Age: Option<u32>,
    pub Mask: Option<String>,
    pub Status: Option<String>,
}
