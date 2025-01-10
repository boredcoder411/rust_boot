#![no_std]
#![no_main]

mod fail;
mod video;
mod mbr;
mod disk;

use core::{arch::global_asm, slice};
use crate::fail::UnwrapOrFail;

global_asm!(include_str!("boot.s"));

extern "C" {
    static _partition_table: u8;
    static _second_stage_start: u8;
}

unsafe fn partition_table_raw() -> *const u8 {
    unsafe { &_partition_table }
}

fn second_stage_start() -> *const () {
    let ptr: *const u8 = unsafe { &_second_stage_start };
    ptr as *const ()
}

#[no_mangle]
pub extern "C" fn first_stage(disk_number: u16) -> ! {
    let partition_table = unsafe { slice::from_raw_parts(partition_table_raw(), 16 * 4) };
    let second_stage_partition = mbr::get_partition(partition_table, 0);

    let entry_point_address = second_stage_start() as u32;

    let mut start_lba = second_stage_partition.logical_block_address.into();
    let mut number_of_sectors = second_stage_partition.number_of_sectors.into();
    let mut target_addr = entry_point_address;

    loop {
        let sectors = u32::min(number_of_sectors, 32) as u16;
        let dap = disk::DiskAddressPacket::from_lba(
            start_lba,
            sectors,
            (target_addr & 0b1111) as u16,
            (target_addr >> 4).try_into().unwrap_or_fail(b'a'),
        );
        unsafe {
            dap.perform_load(disk_number);
        }

        start_lba += u64::from(sectors);
        number_of_sectors -= u32::from(sectors);
        target_addr += 512 * u32::from(sectors);

        if number_of_sectors == 0 {
            break;
        }
    }

    let second_stage_entry_point: extern "C" fn(
        disk_number: u16,
        partition_table: *const u8,
    ) = unsafe { core::mem::transmute(entry_point_address as *const ()) };
    let partition_table_start = unsafe { partition_table_raw() };
    second_stage_entry_point(disk_number, partition_table_start);

    fail::fail(b'R');
}
