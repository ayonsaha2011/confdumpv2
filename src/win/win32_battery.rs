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
pub struct Win32_Battery {
    pub Caption: Option<String>,
    pub Description: Option<String>,
    pub InstallDate: Option<WMIDateTime>,
    pub Name: Option<String>,
    pub Status: Option<String>,
    pub Availability: Option<u16>,
    pub ConfigManagerErrorCode: Option<u32>,
    pub ConfigManagerUserConfig: Option<bool>,
    pub CreationClassName: Option<String>,
    pub DeviceID: Option<String>,
    pub ErrorCleared: Option<bool>,
    pub ErrorDescription: Option<String>,
    pub LastErrorCode: Option<u32>,
    pub PNPDeviceID: Option<String>,
    pub PowerManagementCapabilities: Option<Vec<i32>>,
    pub PowerManagementSupported: Option<bool>,
    pub StatusInfo: Option<u16>,
    pub SystemCreationClassName: Option<String>,
    pub SystemName: Option<String>,
    pub BatteryStatus: Option<u16>,
    pub Chemistry: Option<u16>,
    pub DesignCapacity: Option<u32>,
    pub DesignVoltage: Option<u64>,
    pub EstimatedChargeRemaining: Option<u16>,
    pub EstimatedRunTime: Option<u32>,
    pub ExpectedLife: Option<u32>,
    pub FullChargeCapacity: Option<u32>,
    pub MaxRechargeTime: Option<u32>,
    pub SmartBatteryVersion: Option<String>,
    pub TimeOnBattery: Option<u32>,
    pub TimeToFullCharge: Option<u32>,
    pub BatteryRechargeTime: Option<u32>,
    pub ExpectedBatteryLife: Option<u32>
}
