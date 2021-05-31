// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
#![no_std]

mod consts;
mod frame;
pub mod paging;

pub use consts::*;

use x86_64::{
    structures::paging::{OffsetPageTable, PageTable},
    PhysAddr, VirtAddr,
};

use rust_td_layout::runtime::*;

pub fn init() {
    frame::init();
}

/// page_table_memory_base: page_table_memory_base
/// system_memory_size
pub fn setup_paging(page_table_memory_base: u64, system_memory_size: u64) {
    let mut pt = unsafe {
        OffsetPageTable::new(
            &mut *(page_table_memory_base as *mut PageTable),
            VirtAddr::new(PHYS_VIRT_OFFSET as u64),
        )
    };
    paging::create_mapping(
        &mut pt,
        PhysAddr::new(0),
        VirtAddr::new(0),
        system_memory_size,
    );
    paging::cr3_write();
}
