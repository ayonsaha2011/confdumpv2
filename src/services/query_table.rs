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


use wmi::{COMLibrary, Variant, WMIConnection};
use crate::services::{genrate_output};
use std::error::Error;
use std::fmt;
use serde_json;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::win::*;

pub struct QueryResult {
    pub table: String,
    pub result: Vec<Value>
}

#[derive(Debug)]
pub struct CDError {
    details: String
}

impl CDError {
    fn new(msg: &str) -> CDError {
        CDError{details: msg.to_string()}
    }
}

impl fmt::Display for CDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CDError {
    fn description(&self) -> &str {
        &self.details
    }
}



pub fn query(tables: Vec<String>, output_format: String) -> Result<Vec<QueryResult>, CDError> {
    let mut output = Vec::<QueryResult>::new();
    for table in tables {
        println!("table - {}", table);

        let wmi_result = match table.as_str() {
            "Win32_OperatingSystem" => get_wmi_result::<win32_operating_system::Win32_OperatingSystem>(table),
            "Win32_ComputerSystem" => get_wmi_result::<win32_computer_system::Win32_ComputerSystem>(table),
            "Win32_Service" => get_wmi_result::<win32_service::Win32_Service>(table),
            "Win32_SystemDriver" => get_wmi_result::<win32_system_driver::Win32_SystemDriver>(table),
            "Win32_Process" => get_wmi_result::<win32_process::Win32_Process>(table),
            "Win32_NetworkAdapter" => get_wmi_result::<win32_network_adapter::Win32_NetworkAdapter>(table),
            "Win32_TimeZone" => get_wmi_result::<win32_timezone::Win32_TimeZone>(table),
            "Win32_Processor" => get_wmi_result::<win32_processor::Win32_Processor>(table),
            "Win32_BIOS" => get_wmi_result::<win32_bios::Win32_BIOS>(table),
            "Win32_PhysicalMemory" => get_wmi_result::<win32_physical_memory::Win32_PhysicalMemory>(table),
            "Win32_DiskDrive" => get_wmi_result::<win32_disk_drive::Win32_DiskDrive>(table),
            "Win32_BaseBoard" => get_wmi_result::<win32_base_board::Win32_BaseBoard>(table),
            _ => return Err(CDError::new(&*format!("Oops! table - {} not implemented yet", table)))
        };
        output.push(wmi_result);
    }

    Ok(output)
}

pub fn get_wmi_result<T: for<'de> Deserialize<'de> + Serialize>(table: String) -> QueryResult {
    let mut results_value = Vec::<Value>::new();
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();

    let results: Vec<T> = match wmi_con.query() {
        Ok(result)  => result,
        Err(e) => {println!("{:#?}", e); panic!(e) },
    };
    for result in results {
        match serde_json::to_string(&result) {
            Ok(json_str)  => {
                let json_value: Value = serde_json::from_str(&json_str).unwrap();
                results_value.push( json_value);
            },
            Err(e) => println!("{:#?}", e),
        }
    }
    QueryResult{ table, result: results_value }
}