#![no_std]
#![no_main]

use uefi::{Status};
mod uart_debug;

#[no_mangle]
pub extern "efiapi" fn efi_main() -> Status {
    unsafe {
        uart_debug::uart_init();
        uart_debug::log("UART communication established on Pi 5 D0.\n");
    }
    Status::SUCCESS
}