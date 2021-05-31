// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use core::convert::TryInto;

use tdx_tdcall::tdx;

use ring::digest;

const SHA384_DIGEST_SIZE: usize = 48;
const TPM_ALG_SHA384: u16 = 0xc;

#[allow(dead_code)]
#[repr(packed)]
struct TpmuHa {
    sha384: [u8; SHA384_DIGEST_SIZE],
}

#[allow(dead_code)]
#[repr(packed)]
struct TpmtHa {
    hash_alg: u16,
    digest: TpmuHa,
}

#[allow(dead_code)]
#[repr(packed)]
struct TpmlDigestValues {
    count: u32,
    digests: [TpmtHa; 1],
}

#[allow(dead_code)]
#[repr(packed)]
struct TcgPcrEvent2Header {
    pcr_index: u32,
    event_type: u32,
    digest: TpmlDigestValues,
    event_size: u32,
}

pub fn extend_rtmr(data: &[u8; SHA384_DIGEST_SIZE], pcr_index: u32) {
    let digest = tdx::TdxDigest { data: *data };

    let mr_index = match pcr_index {
        0 => {
            log::info!("PCR[0] should be extended vith RDMR\n");
            0xFF
        }
        1 | 7 => 0,
        2..=6 => 1,
        8..=15 => 2,
        _ => {
            log::info!("invalid pcr_index 0x{:x}\n", pcr_index);
            0xFF
        }
    };
    if mr_index >= 3 {
        return;
    }

    tdx::tdcall_extend_rtmr(&digest, mr_index);
}

pub fn create_td_event(pcr_index: u32, event_type: u32, event_data: &[u8], hash_data: &[u8]) {
    log::info!("calc td_hob digest ...\n");

    let event_data_size = event_data.len();

    let hash_value = digest::digest(&digest::SHA384, hash_data);
    let hash_value = hash_value.as_ref();

    assert_eq!(hash_value.len(), SHA384_DIGEST_SIZE);

    log::info!("extend_rtmr ...\n");
    extend_rtmr(hash_value.try_into().unwrap(), pcr_index);

    let hash384_value: [u8; SHA384_DIGEST_SIZE] = hash_value.try_into().unwrap();
    let _event2_header = TcgPcrEvent2Header {
        pcr_index,
        event_type,
        digest: TpmlDigestValues {
            count: 1,
            digests: [TpmtHa {
                hash_alg: TPM_ALG_SHA384,
                digest: TpmuHa {
                    sha384: hash384_value,
                },
            }],
        },
        event_size: event_data_size as u32,
    };

    log::info!("create_td_event done\n");
    // TBD - record event log to a common buffer.
}
