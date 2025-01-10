use core::arch::asm;

#[repr(C)]
#[allow(dead_code)]
pub struct DiskAddressPacket {
    packet_size: u8,
    reserved: u8, /// Always 0
    number_of_sectors: u16,
    offset: u16,
    segment: u16,
    start_lba: u64,
}

impl DiskAddressPacket {
    pub fn from_lba(
        start_lba: u64,
        number_of_sectors: u16,
        target_offset: u16,
        target_segment: u16,
    ) -> Self {
        Self {
            packet_size: 0x10,
            reserved: 0,
            number_of_sectors,
            offset: target_offset,
            segment: target_segment,
            start_lba,
        }
    }

    pub unsafe fn perform_load(&self, disk_number: u16) {
        let self_addr = self as *const Self as u16;
        unsafe {
            asm!(
                "push 'z'", // error code `z`, passed to `fail` on error
                "mov {1:x}, si", // backup the `si` register, whose contents are required by LLVM
                "mov si, {0:x}",
                "int 0x13",
                "jnc 2f", // carry is set on fail
                "call fail",
                "2:",
                "pop si", // remove error code again
                "mov si, {1:x}", // restore the `si` register to its prior state
                in(reg) self_addr,
                out(reg) _,
                in("ax") 0x4200u16,
                in("dx") disk_number,
            );
        }
    }
}
