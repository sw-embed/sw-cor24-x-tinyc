/// COR24-TB target hardware constants.
pub struct TargetConfig {
    /// Word size in bytes (3 for COR24).
    pub word_size: i32,
    /// Stack pointer initial value.
    pub stack_init: u32,
    /// LED register address (GPIO).
    pub led_addr: u32,
    /// UART data register address.
    pub uart_data_addr: u32,
    /// UART status register address.
    pub uart_status_addr: u32,
    /// Interrupt enable register address.
    pub int_enable_addr: u32,
}

impl TargetConfig {
    /// Default COR24-TB configuration.
    pub fn cor24_tb() -> Self {
        Self {
            word_size: 3,
            stack_init: 0xFEEC00,
            led_addr: 0xFF0000,
            uart_data_addr: 0xFF0100,
            uart_status_addr: 0xFF0101,
            int_enable_addr: 0xFF0010,
        }
    }
}
