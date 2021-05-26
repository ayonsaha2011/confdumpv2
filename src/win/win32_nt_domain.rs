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
pub struct Win32_NTDomain {
    pub DomainGuid: Option<String>,
    pub ClientSiteName: Option<String>,
    pub CreationClassName: Option<String>,
    pub DomainControllerAddressType: Option<i32>,
    pub DSWritableFlag: Option<bool>,
    pub Description: Option<String>,
    pub PrimaryOwnerName: Option<String>,
    pub Status: Option<String>,
    pub Caption: Option<String>,
    pub DSPrimaryDomainControllerFlag: Option<bool>,
    pub DSDnsDomainFlag: Option<bool>,
    pub DnsForestName: Option<String>,
    pub DSKerberosDistributionCenterFlag: Option<bool>,
    pub DcSiteName: Option<String>,
    pub DomainControllerAddress: Option<String>,
    pub DSTimeServiceFlag: Option<bool>,
    pub PrimaryOwnerContact: Option<String>,
    pub Roles: Option<Vec<String>>,
    pub DSDirectoryServiceFlag: Option<bool>,
    pub DSGlobalCatalogFlag: Option<bool>,
    pub Name: Option<String>,
    pub DSDnsControllerFlag: Option<bool>,
    pub InstallDate: Option<WMIDateTime>,
    pub NameFormat: Option<String>,
    pub DomainName: Option<String>,
    pub DSDnsForestFlag: Option<bool>,
    pub DomainControllerName: Option<String>,
}
