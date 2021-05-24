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


use std::io::{Write, Error};
use std::fs::OpenOptions;
use crate::services::query_table::QueryResult;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::*;

pub fn genrate(output_format: String, output_path: Option<&str>, results: Vec<QueryResult>) {

    if output_path.is_some() {
        match output_format.as_str() {
            "text" => genrate_text(output_path.unwrap(), results),
            "xml" => genrate_xml(output_path.unwrap(), results),
            "json" => genrate_json(output_path.unwrap(), results),
            _ => println!("Input output format not equal any value"),
        }
    } else {
        genrate_print(results)
    }
}
pub fn genrate_print(results: Vec<QueryResult>) {
    for result in results {

        for json_value in result.result {
            println!(" ");
            println!("{}", result.table);
            for (key, value) in json_value.as_object().unwrap() {
                println!(" - ");
                if value.is_array() {
                    println!("  {}: ", key);
                    let n_value = value.as_array().unwrap();
                    for n_v in n_value {
                        println!("      - {}", format!("{:#}", n_v));
                    }
                } else {
                    println!("  {}: {}", key, format!("{:#}", value));
                }
            }
        }
    }
}
pub fn genrate_text(output_path: &str, results: Vec<QueryResult>) {
    let mut file = OpenOptions::new().create(true).append(true).open(output_path).expect(
        "cannot open file");
    let write_line = || -> Result<(), Error> {
        for result in results {
            writeln!(file, " ")?;
            writeln!(file, "{}", result.table)?;
            for json_value in result.result {
                writeln!(file, " - ")?;
                for (key, value) in json_value.as_object().unwrap() {
                    if value.is_array() {
                        writeln!(file, "  {}: ", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            if n_v.is_string() {
                                writeln!(file, "      - {}", n_v.as_str().unwrap())?;
                            } else {
                                writeln!(file, "      - {}", format!("{:#}", n_v))?;
                            }
                        }
                    } else if value.is_string() {
                        writeln!(file, "  {}: {}", key, value.as_str().unwrap())?;
                    } else {
                        writeln!(file, "  {}: {}", key, format!("{:#}", value))?;
                    }
                }
            }
        }

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn genrate_xml(output_path:  &str, results: Vec<QueryResult>) {
    let mut file = OpenOptions::new().create(true).append(true).open(output_path).expect(
        "cannot open file");
    let new_uuid = Uuid::new_v4();
    let local_time: DateTime<Local> = Local::now();
    let date = &local_time.format("%d%m%Y%H%M%S").to_string();
    let write_line = || -> Result<(), Error> {
        writeln!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(file, "<collect date={:?} timestamp={:?} uuid=\"{}\">", date, local_time, new_uuid)?;
        for result in results {
            writeln!(file, "    <query class=\"{}\">", result.table)?;
            for json_value in result.result {
                writeln!(file, "        <item>")?;
                for (key, value) in json_value.as_object().unwrap() {
                    if value.is_array() {
                        writeln!(file, "            <attr name=\"{}\" operator=\"=\">", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            if n_v.is_string() {
                                writeln!(file, "                <element>{}</element>", n_v.as_str().unwrap())?;
                            } else {
                                writeln!(file, "                <element>{}</element>", format!("{:#}", n_v))?;
                            }
                        }
                        writeln!(file, "            </attr>")?;
                    } else if value.is_string() {
                        writeln!(file, "            <attr name=\"{}\" operator=\"=\">{}</attr>", key, value.as_str().unwrap())?;
                    } else {
                        writeln!(file, "            <attr name=\"{}\" operator=\"=\">{}</attr>", key, format!("{:#}", value))?;
                    }
                }
                writeln!(file, "        </item>")?;
            }
            writeln!(file, "    </query>")?;
        }
        writeln!(file, "</collect>")?;

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    }
}
pub fn genrate_json(output_path:  &str, results: Vec<QueryResult>) {
    let mut file = OpenOptions::new().create(true).append(true).open(output_path).expect(
        "cannot open file");
    let write_line = || -> Result<(), Error> {
        writeln!(file, "{{")?;
        for result in results {
            writeln!(file, "    {:?}: [", result.table)?;
            for json_value in result.result {
                writeln!(file, "        {{")?;
                for (key, value) in json_value.as_object().unwrap() {
                    if value.is_array() {
                        writeln!(file, "        {:?}: [", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            writeln!(file, "                {}", format!("{:#}", n_v))?;
                        }
                        writeln!(file, "        ],")?;
                    } else {
                        writeln!(file, "        {:?}: {:?},", key, format!("{:#}", value))?;
                    }
                }
                writeln!(file, "        }},")?;
            }
            writeln!(file, "    ],")?;
        }
        writeln!(file, "}}")?;

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    }
}

