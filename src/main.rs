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

extern crate clap;
extern crate wmi;

use std::io::Write;
use clap::{Arg, App};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

mod win;
mod services;

use crate::services::genrate_output;
use crate::services::query_table;


fn main() {
    // basic app information
    let app = App::new("Confdumpv2")
        .version("0.0.1")
        .about("Confdump-Agent collects system configuration, both static (as set by an administrator) and runtime (currently in use by the system).")
        .author("Frédéric Hay");

    let table_option = Arg::with_name("table")
        .long("table")
        .takes_value(true)
        .help("tables to query (can be specified multiple times);
                        format: [dumper.]table");

    let tables_option = Arg::with_name("tables-from")
        .long("tables-from")
        .takes_value(true)
        .help("read a list of tables to query from this file");

    let output_format_option = Arg::with_name("output-format")
        .long("output-format")
        .default_value("text")
        .takes_value(true)
        .help("sets output format to one of: text, xml
  --output-file arg           output to a file instead of standard output");


    let output_file_option = Arg::with_name("output-file")
        .long("output-file")
        .takes_value(true)
        .help("output to a file instead of standard output");

    let app = app.arg(table_option)
        .arg(tables_option)
        .arg(output_format_option)
        .arg(output_file_option);
    let matches = app.get_matches();
    let table = matches.value_of("table");
    let tables_from = matches.value_of("tables-from");
    let output_file = matches.value_of("output-file");
    let output_format = matches.value_of("output-format").unwrap();


    let mut tables_list = Vec::<String>::new();

    if table.is_some() {
        tables_list.push(table.unwrap().to_string());
    } else if tables_from.is_some() {
        let tables_from_path = tables_from.unwrap().to_string();
        println!("tables_from_path {}", &tables_from_path);
        let file = File::open(tables_from_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            tables_list.push(line.unwrap());
        }
    }
    if tables_list.len() > 0 {
        let results = query_table::query(tables_list, output_format.to_string());
        match results {
            Ok(query_results) => genrate_output::genrate(output_format.to_string(), output_file, query_results),
            Err(e) => println!("{:?}", e),
        }
    }
}
