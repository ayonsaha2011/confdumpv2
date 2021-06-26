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
pub struct Win32_TimeZone {
    pub Description: Option<String>,
    pub StandardDay: Option<u32>,
    pub StandardHour: Option<u32>,
    pub StandardMillisecond: Option<u32>,
    pub StandardMonth: Option<u32>,
    pub StandardName: Option<String>,
    pub DaylightMillisecond: Option<u32>,
    pub StandardSecond: Option<u32>,
    pub DaylightDayOfWeek: Option<u8>,
    pub DaylightName: Option<String>,
    pub StandardMinute: Option<u32>,
    pub DaylightHour: Option<u32>,
    pub StandardDayOfWeek: Option<u8>,
    pub DaylightMinute: Option<u32>,
    pub StandardYear: Option<u32>,
    pub DaylightDay: Option<u32>,
    pub DaylightYear: Option<u32>,
    pub Caption: Option<String>,
    pub SettingID: Option<String>,
    pub Bias: Option<i32>,
    pub DaylightBias: Option<i32>,
    pub DaylightMonth: Option<u32>,
    pub DaylightSecond: Option<u32>,
    pub StandardBias: Option<u32>,
}
