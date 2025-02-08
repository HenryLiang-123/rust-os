#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this is the first function executed by the CPU
    // this function must not return 
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // freezes execution, prevents further undefined behavior
}



