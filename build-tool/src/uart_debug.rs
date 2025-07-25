// UART constants for Raspberry Pi 5 (including D0 stepping variations)
// Try these addresses in order if one doesn't work
const UART0_BASE_PRIMARY: u64 = 0x107D001000;   // Standard Pi 5 address

// Use primary address by default
const UART0_BASE: u64 = UART0_BASE_PRIMARY;
const UART_DR: u64 = UART0_BASE + 0x00;      // Data register
const UART_FR: u64 = UART0_BASE + 0x18;      // Flag register
// const UART_IBRD: u64 = UART0_BASE + 0x24;    // Integer baud rate divisor
// const UART_FBRD: u64 = UART0_BASE + 0x28;    // Fractional baud rate divisor
const UART_LCRH: u64 = UART0_BASE + 0x2C;    // Line control register
const UART_CR: u64 = UART0_BASE + 0x30;      // Control register

// Flag register bits
const UART_FR_TXFF: u32 = 1 << 5;  // Transmit FIFO full

// Control register bits
const UART_CR_UARTEN: u32 = 1 << 0;  // UART enable
const UART_CR_TXE: u32 = 1 << 8;     // Transmit enable
const UART_CR_RXE: u32 = 1 << 9;     // Receive enable

// Line control register bits
const UART_LCRH_WLEN_8: u32 = 0x60;  // 8-bit word length
const UART_LCRH_FEN: u32 = 1 << 4;   // Enable FIFOs

/// Panic handler required for no_std
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Read from a memory-mapped register
pub unsafe fn read_reg(addr: u64) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

/// Write to a memory-mapped register
pub unsafe fn write_reg(addr: u64, value: u32) {
    core::ptr::write_volatile(addr as *mut u32, value);
}

/// Initialize the UART for basic communication
pub unsafe fn uart_init() {
    // Try to detect if UART is accessible by reading a register
    // let initial_cr = read_reg(UART_CR);
    
    // Disable UART
    write_reg(UART_CR, 0);
    
    // Verify the write worked (basic accessibility test)
    let disabled_cr = read_reg(UART_CR);
    if disabled_cr != 0 {
        // UART registers might not be accessible at this address
        return;
    }
    
    // Set baud rate to 115200 
    // Pi 5 D0 might use different clock - try standard values first
    // write_reg(UART_IBRD, 26);  // Integer part
    // write_reg(UART_FBRD, 3);   // Fractional part
    
    // Set line control: 8-bit, no parity, 1 stop bit, FIFOs enabled
    write_reg(UART_LCRH, UART_LCRH_WLEN_8 | UART_LCRH_FEN);
    
    // Enable UART, TX, and RX
    write_reg(UART_CR, UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE);
    
    // Longer delay for Pi 5 D0 to stabilize
    for _ in 0..10000 {
        core::ptr::read_volatile(&0 as *const i32);
    }
}

/// Write a single character to UART
pub unsafe fn uart_putc(c: u8) {
    // Wait until transmit FIFO is not full
    while (read_reg(UART_FR) & UART_FR_TXFF) != 0 {
        // Busy wait
    }
    
    // Write character to data register
    write_reg(UART_DR, c as u32);
}

/// Write a string to UART
pub unsafe fn log(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
        
        // Convert \n to \r\n for proper line endings
        if byte == b'\n' {
            uart_putc(b'\r');
        }
    }
}
