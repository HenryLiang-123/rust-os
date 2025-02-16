#![no_std]
#![no_main]

use core::panic::PanicInfo;


// #[no_mangle] // don't mangle the name of this function
// pub extern "C" fn _start() -> ! {
//     // this is the first function executed by the CPU
//     // this function must not return 
//     loop {}
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // freezes execution, prevents further undefined behavior
}

static HELLO: &[u8] = b"Hello World!"; //byte string

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
