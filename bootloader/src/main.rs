#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // ARM64 Boot Flow
    // 1. Current EL check (Exception Level)
    // 2. Setup Stack Pointer
    // 3. Parse Device Tree Blob (DTB) provided by QEMU (x0 register)
    
    // Initialize GOP Graphics (Mocked call to display_driver)
    // let mut fb = GopFrameBuffer::new(0x3000_0000, 800, 800, 600);
    // fb.clear_screen(0xFF00_0000); // Black

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
