// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use tdx_tdcall::tdx;

const PAGE_ACCEPT_CHUNK_SIZE: u64 = 0x2000;

fn td_accept_page(address: u64, pages: u64) {
    for i in 0..pages {
        tdx::tdcall_accept_page(address + i * 0x1000);
    }
}

pub fn mp_accept_memory_resource_range(address: u64, size: u64) {
    log::info!(
        "mp_accept_memory_resource_range: 0x{:x} - 0x{:x} ... (wait for 1 min)\n",
        address,
        size
    );

    let pages = PAGE_ACCEPT_CHUNK_SIZE >> 12;

    for i in 0..(size / PAGE_ACCEPT_CHUNK_SIZE) {
        // TBD accept failed if remove this!
        if (address + i * PAGE_ACCEPT_CHUNK_SIZE) % 0x800000 == 0 {
            log::info!(
                "accept pages 0x{:X}\n",
                address + i * PAGE_ACCEPT_CHUNK_SIZE
            );
        }
        td_accept_page(address + i * PAGE_ACCEPT_CHUNK_SIZE, pages);
    }

    log::info!("mp_accept_memory_resource_range: done\n");
}
