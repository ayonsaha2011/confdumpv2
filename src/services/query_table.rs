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
use crate::win::{win32_operating_system::Win32_OperatingSystem, win32_computer_system::Win32_ComputerSystem, win32_service::Win32_Service};
use std::error::Error;
use std::fmt;
use serde_json;
use serde_json::Value;

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

    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();

    for table in tables {
        println!("table - {}", table);

        if  table == "Win32_OperatingSystem" {
            let mut results_value = Vec::<Value>::new();
            let results: Vec<Win32_OperatingSystem> = match wmi_con.query() {
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
            output.push(QueryResult { table, result: results_value });

        } else if  table == "Win32_ComputerSystem" {
            let mut results_value = Vec::<Value>::new();
            let results: Vec<Win32_ComputerSystem> = match wmi_con.query() {
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
            output.push(QueryResult { table, result: results_value });

        } else if  table == "Win32_Service" {
            let mut results_value = Vec::<Value>::new();
            let results: Vec<Win32_Service> = match wmi_con.query() {
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
            output.push(QueryResult { table, result: results_value });

        }else {
            let error_str = format!("Oops! table - {} not implemented yet", table);
            // println!(error_str);
            return Err(CDError::new(&*error_str));
        }
    }

    Ok(output)
}


