use core::arch::asm;

#[no_mangle]
pub extern "C" fn print_char(c: u8) {
    let ax = u16::from(c) | 0x0e00;
    unsafe {
        asm!("push bx", "mov bx, 0", "int 0x10", "pop bx", in("ax") ax);
    }
}
