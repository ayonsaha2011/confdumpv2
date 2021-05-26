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

pub mod win32_operating_system;
pub mod win32_computer_system;
pub mod win32_service;
pub mod win32_timezone;
pub mod win32_system_driver;
pub mod win32_processor;
pub mod win32_process;
pub mod win32_network_adapter;
pub mod win32_base_board;
pub mod win32_bios;
pub mod win32_disk_drive;
pub mod win32_physical_memory;
pub mod win32_account;
pub mod win32_system_slot;
pub mod win32_network_connection;
pub mod win32_group;
pub mod win32_environment;
pub mod win32_nt_domain;
pub mod win32_ip4_route_table;
pub mod win32_startup_command;