// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::{
    env, format, fs,
    path::{Path, PathBuf},
    process::Command,
};

use rust_td_layout::build_time;

fn nasm(file: &Path, arch: &str, out_file: &Path, args: &[&str]) -> Command {
    let oformat = match arch {
        "x86_64" => ("win64"),
        "x86" => ("win32"),
        "bin" => ("bin"),
        _ => panic!("unsupported arch: {}", arch),
    };
    let mut c = Command::new("nasm");
    let _ = c
        .arg("-o")
        .arg(out_file.to_str().expect("Invalid path"))
        .arg("-f")
        .arg(oformat)
        .arg(file);
    for arg in args {
        let _ = c.arg(*arg);
    }
    c
}

fn run_command(mut cmd: Command) {
    eprintln!("running {:?}", cmd);
    let status = cmd.status().unwrap_or_else(|e| {
        panic!("failed to execute [{:?}]: {}", cmd, e);
    });
    if !status.success() {
        panic!("execution failed");
    }
}

fn main() {
    // tell cargo when to re-run the script
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        Path::new("ResetVector/ResetVector.asm").to_str().unwrap()
    );

    let old_current_dir = env::current_dir().unwrap();
    let new_current_dir = old_current_dir.join("ResetVector");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = PathBuf::from(out_dir).join("ResetVector.bin");
    let copy_to_dir = out_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let copy_to_file = copy_to_dir.join("ResetVector.bin");

    eprintln!("out_file is     {}", out_file.to_str().unwrap());
    eprintln!("copy_to_file is {}", copy_to_file.to_str().unwrap());

    let use_tdx_emulation_arg = format!(
        "-DUSE_TDX_EMULATION={}",
        if tdx_tdcall::USE_TDX_EMULATION {
            1u8
        } else {
            0u8
        }
    );
    let td_shim_ipl_base_arg = format!("-DTOP_OF_BFV=0x{:X}", build_time::TD_SHIM_IPL_BASE);
    let td_mailbax_base_arg = format!("-DTD_MAILBOX_BASE=0x{:X}", build_time::TD_SHIM_MAILBOX_BASE);
    let td_mailbax_size_arg = format!("-DTD_MAILBOX_SIZE=0x{:X}", build_time::TD_SHIM_MAILBOX_SIZE);
    let td_shim_hob_base_arg = format!("-DTD_HOB_BASE=0x{:X}", build_time::TD_SHIM_HOB_BASE);
    let td_shim_hob_size_arg = format!("-DTD_HOB_SIZE=0x{:X}", build_time::TD_SHIM_HOB_SIZE);
    let td_shim_tmp_stack_base_arg = format!(
        "-DTEMP_STACK_BASE=0x{:X}",
        build_time::TD_SHIM_TEMP_STACK_BASE
    );
    let td_shim_tmp_stack_size_arg = format!(
        "-DTEMP_STACK_SIZE=0x{:X}",
        build_time::TD_SHIM_TEMP_STACK_SIZE
    );
    let td_shim_tmp_heap_base_arg =
        format!("-DTEMP_RAM_BASE=0x{:X}", build_time::TD_SHIM_TEMP_HEAP_BASE);
    let td_shim_tmp_heap_size_arg =
        format!("-DTEMP_RAM_SIZE=0x{:X}", build_time::TD_SHIM_TEMP_HEAP_SIZE);

    let _ = env::set_current_dir(new_current_dir.as_path());
    run_command(nasm(
        Path::new("ResetVector.nasm"),
        "bin",
        out_file.as_path(),
        &[
            &use_tdx_emulation_arg,
            &td_shim_ipl_base_arg,
            &td_mailbax_base_arg,
            &td_mailbax_size_arg,
            &td_shim_hob_base_arg,
            &td_shim_hob_size_arg,
            &td_shim_tmp_stack_base_arg,
            &td_shim_tmp_stack_size_arg,
            &td_shim_tmp_heap_base_arg,
            &td_shim_tmp_heap_size_arg,
        ],
    ));

    let _ = env::set_current_dir(old_current_dir.as_path());
    let _ = fs::copy(&out_file, &copy_to_file).unwrap();
}
