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
pub struct Win32_NetworkAdapterConfiguration {
    pub WINSPrimaryServer: Option<String>,
    pub DHCPLeaseExpires: Option<WMIDateTime>,
    pub IPAddress: Option<Vec<String>>,
    pub SettingID: Option<String>,
    pub DHCPServer: Option<String>,
    pub IPPortSecurityEnabled: Option<bool>,
    pub DHCPLeaseObtained: Option<WMIDateTime>,
    pub KeepAliveTime: Option<u32>,
    pub KeepAliveInterval: Option<u32>,
    pub DNSDomainSuffixSearchOrder: Option<Vec<String>>,
    pub IPEnabled: Option<bool>,
    pub IPXMediaType: Option<u32>,
    pub IPXEnabled: Option<bool>,
    pub MACAddress: Option<String>,
    pub DNSDomain: Option<String>,
    pub TcpMaxDataRetransmissions: Option<u32>,
    pub IGMPLevel: Option<u8>,
    pub IPSecPermitIPProtocols: Option<Vec<String>>,
    pub IPConnectionMetric: Option<u32>,
    pub PMTUDiscoveryEnabled: Option<bool>,
    pub DatabasePath: Option<String>,
    pub WINSScopeID: Option<String>,
    pub Description: Option<String>,
    pub TcpNumConnections: Option<u32>,
    pub IPSecPermitUDPPorts: Option<Vec<String>>,
    pub IPFilterSecurityEnabled: Option<bool>,
    pub IPXFrameType: Option<Vec<i32>>,
    pub DomainDNSRegistrationEnabled: Option<bool>,
    pub WINSSecondaryServer: Option<String>,
    pub DefaultIPGateway: Option<Vec<String>>,
    pub DNSServerSearchOrder: Option<Vec<String>>,
    pub TcpWindowSize: Option<u16>,
    pub DefaultTTL: Option<u8>,
    pub ServiceName: Option<String>,
    pub TcpipNetbiosOptions: Option<u32>,
    pub DefaultTOS: Option<u8>,
    pub WINSHostLookupFile: Option<String>,
    pub GatewayCostMetric: Option<Vec<i32>>,
    pub DNSEnabledForWINSResolution: Option<bool>,
    pub DHCPEnabled: Option<bool>,
    pub IPXVirtualNetNumber: Option<String>,
    pub NumForwardPackets: Option<u32>,
    pub IPXNetworkNumber: Option<Vec<String>>,
    pub TcpMaxConnectRetransmissions: Option<u32>,
    pub IPSecPermitTCPPorts: Option<Vec<String>>,
    pub FullDNSRegistrationEnabled: Option<bool>,
    pub InterfaceIndex: Option<u32>,
    pub IPUseZeroBroadcast: Option<bool>,
    pub DNSHostName: Option<String>,
    pub Index: Option<u32>,
    pub Caption: Option<String>,
    pub MTU: Option<u32>,
    pub ArpUseEtherSNAP: Option<bool>,
    pub PMTUBHDetectEnabled: Option<bool>,
    pub WINSEnableLMHostsLookup: Option<bool>,
    pub ArpAlwaysSourceRoute: Option<bool>,
    pub TcpUseRFC1122UrgentPointer: Option<bool>,
    pub IPXAddress: Option<String>,
    pub DeadGWDetectEnabled: Option<bool>,
    pub ForwardBufferMemory: Option<u32>,
    pub IPSubnet: Option<Vec<String>>,
}
