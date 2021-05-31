// Copyright © 2019 Intel Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

mod memslice;

use uefi_pi::pi::hob_lib;

use rust_td_layout::runtime::*;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use core::panic::PanicInfo;

use core::ffi::c_void;

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(_info: &PanicInfo) -> ! {
    log::info!("panic ... {:?}\n", _info);
    loop {}
}

#[alloc_error_handler]
#[allow(clippy::empty_loop)]
fn alloc_error(_info: core::alloc::Layout) -> ! {
    log::info!("alloc_error ... {:?}\n", _info);
    loop {}
}

fn init_heap(heap_start: usize, heap_size: usize) {
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[no_mangle]
#[cfg_attr(target_os = "uefi", export_name = "efi_main")]
pub extern "win64" fn _start(hob: *const c_void) -> ! {
    tdx_logger::init();
    log::info!("Starting rust-td-payload hob - {:p}\n", hob);

    tdx_exception::setup_exception_handlers();
    log::info!("setup_exception_handlers done\n");

    // let hob_buffer = unsafe {
    //     core::slice::from_raw_parts(hob as *const u8, TD_PAYLOAD_HOB_SIZE as usize)
    // };

    let hob_buffer =
        memslice::get_dynamic_mem_slice_mut(memslice::SliceType::TdPayloadHobSlice, hob as usize);

    let hob_size = hob_lib::get_hob_total_size(hob_buffer).unwrap();
    let hob_list = &hob_buffer[..hob_size];
    hob_lib::dump_hob(hob_list);

    init_heap(
        (hob_lib::get_system_memory_size_below_4gb(hob_list) as usize
            - (TD_PAYLOAD_HOB_SIZE as usize + TD_PAYLOAD_STACK_SIZE as usize)
            - TD_PAYLOAD_HEAP_SIZE as usize),
        TD_PAYLOAD_HEAP_SIZE as usize,
    );

    // Test
    unsafe {
        let pointer: *const u32 = 0x10000000000usize as *const core::ffi::c_void as *const u32;
        let data = *pointer;
        log::info!("test - data: {:x}", data);
    }

    loop {}
}
