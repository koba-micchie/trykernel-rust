
pub const SRAM_START: u32 = 0x2000_0000;
pub const SRAM_SIZE:  u32 = 256 * 1024;
pub const INITIAL_SP: u32 = SRAM_START + SRAM_SIZE;

// APB Peripheral
// Clocks
pub const CLOCKS_BASE: u32 = 0x4000_8000;
pub const CLK_GPOUT0:  u32 = CLOCKS_BASE + 0x00 as u32;
pub const CLK_GPOUT1:  u32 = CLOCKS_BASE + 0x0C as u32;
pub const CLK_GPOUT2:  u32 = CLOCKS_BASE + 0x18 as u32;
pub const CLK_GPOUT3:  u32 = CLOCKS_BASE + 0x24 as u32;
pub const CLK_REF:     u32 = CLOCKS_BASE + 0x30 as u32;
pub const CLK_SYS:     u32 = CLOCKS_BASE + 0x3C as u32;
pub const CLK_PERI:    u32 = CLOCKS_BASE + 0x48 as u32;
pub const CLK_USB:     u32 = CLOCKS_BASE + 0x54 as u32;
pub const CLK_ADC:     u32 = CLOCKS_BASE + 0x60 as u32;
pub const CLK_RTC:     u32 = CLOCKS_BASE + 0x6C as u32;
pub const CLK_SYS_RESUS_CTRL: u32 = CLOCKS_BASE + 0x78 as u32;
pub const CLK_RESUS_STATUS:   u32 = CLOCKS_BASE + 0x7C as u32;

pub const CLK_X_CTRL:     u32 = 0x00 ;
pub const CLK_X_DIV:      u32 = 0x04 ;
pub const CLK_X_SELECTED: u32 = 0x08 ;

pub const CLK_SYS_CTRL_AUXSRC: u32 = 0x0000_00e0;
pub const CLK_SYS_CTRL_SRC:    u32 = 0x0000_0001;
pub const CLK_REF_CTRL_SRC:    u32 = 0x0000_0003;
pub const CLK_CTRL_ENABLE:     u32 = 0x0000_0800;

pub const CLK_SYS_CTRL_SRC_AUX: u32 = 0x1;

pub const CLK_KIND_GPOUT0: usize = 0;
pub const CLK_KIND_GPOUT1: usize = 1;
pub const CLK_KIND_GPOUT2: usize = 2;
pub const CLK_KIND_GPOUT3: usize = 3;
pub const CLK_KIND_REF:    usize = 4;
pub const CLK_KIND_SYS:    usize = 5;
pub const CLK_KIND_PERI:   usize = 6;
pub const CLK_KIND_USB:    usize = 7;
pub const CLK_KIND_ADC:    usize = 8;
pub const CLK_KIND_RTC:    usize = 9;

// Reset Controller
pub const RESETS_BASE:        u32 = 0x4000_C000;
pub const RESETS_RESET:       u32 = RESETS_BASE + 0x00 as u32;
pub const RESETS_WDSEL:       u32 = RESETS_BASE + 0x04 as u32;
pub const RESETS_RESET_DONE:  u32 = RESETS_BASE + 0x08 as u32;

pub const RESETS_RESET_ADC:   u32 = 0x0000_0001;
pub const RESETS_RESET_I2C0:  u32 = 0x0000_0008;
pub const RESETS_RESET_I2C1:  u32 = 0x0000_0010;

// GPIO
pub const IO_BANK0_BASE:      u32 = 0x4001_4000;
pub fn gpio_ctrl(n: u32) -> u32 {
    return IO_BANK0_BASE + 0x04 as u32 + (n * 8) as u32;
}

pub const GPIO_CTRL_FUNCSEL_I2C:  u8 = 3;
pub const GPIO_CTRL_FUNCSEL_NULL: u8 = 31;

pub const PADS_BANK0_BASE:    u32 = 0x4001_C000;
pub fn gpio(n: u32) -> u32 {
    return PADS_BANK0_BASE + 0x4 as u32 + (n * 4) as u32;
}

pub const GPIO_OD: u32         = 1 << 7;
pub const GPIO_IE: u32         = 1 << 6;
pub const GPIO_DRIVE_2MA:  u32 = 0 << 4;
pub const GPIO_DRIVE_4MA:  u32 = 1 << 4;
pub const GPIO_DRIVE_8MA:  u32 = 2 << 4;
pub const GPIO_DRIVE_12MA: u32 = 3 << 4;
pub const GPIO_PUE:        u32 = 1 << 3;
pub const GPIO_PDE:        u32 = 1 << 2;
pub const GPIO_SHEMITT:    u32 = 1 << 1;
pub const GPIO_SLEWDAST:   u32 = 1 << 0;

// Crystal Oscillator(XOSC)
pub const XOSC_BASE:      u32 = 0x4002_4000;
pub const XOSC_CTRL:      u32 = XOSC_BASE + 0x00 as u32;
pub const XOSC_STATUS:    u32 = XOSC_BASE + 0x04 as u32;
pub const XOSC_STARTUP:   u32 = XOSC_BASE + 0x0C as u32;

pub const XOSC_CTRL_ENABLE:        u32 = 0x00FA_B000;
pub const XOSC_CTRL_DISABLE:       u32 = 0x00D1_E000;
pub const XOSC_CTRL_FRANG_1_15MHZ: u32 = 0x0000_0AA0;
pub const XOSC_STATUS_STABLE:      u32 = 0x8000_0000;

// PLL
pub const PLL_SYS_BASE:   u32 = 0x4002_8000;
pub const PLL_USB_BASE:   u32 = 0x4002_C000;

pub const PLL_CS:        u32 = 0x00;
pub const PLL_PWR:       u32 = 0x04;
pub const PLL_FBDIV_INT: u32 = 0x08;
pub const PLL_PRIM:      u32 = 0x0C;

pub const PLL_CS_LOCK:           u32 = 1 << 31;
pub const PLL_PWR_PD:            u32 = 1 << 0;
pub const PLL_PWR_VCOPD:         u32 = 1 << 5;
pub const PLL_PWR_POSTDIVPD:     u32 = 1 << 3;
pub const PLL_PRIM_POSTDIV1_LSB: u32 = 16;
pub const PLL_PRIM_POSTDIV2_LSB: u32 = 12;

// UART
pub const UART0_BASE:    u32 = 0x4003_4000;
pub const UART1_BASE:    u32 = 0x4003_8000;

pub const UARTX_DR:    u32 = 0x000;
pub const UARTX_FR:    u32 = 0x018;
pub const UARTX_IBRD:  u32 = 0x024;
pub const UARTX_FBRD:  u32 = 0x028;
pub const UARTX_LCR_H: u32 = 0x02C;
pub const UARTX_CR:    u32 = 0x030;

pub const UART_CR_RXE:  u32 = 1 << 9;
pub const UART_CR_TXE:  u32 = 1 << 8;
pub const UART_CR_EN:   u32 = 1 << 0;
pub const UART_FR_TXFF: u32 = 1 << 5;

// IOPORT Register
pub const SIO_BASE:     u32 = 0xD000_0000;
pub const GPIO_IN:      u32 = SIO_BASE + 0x04 as u32;
pub const GPIO_OUT:     u32 = SIO_BASE + 0x10 as u32;
pub const GPIO_OUT_SET: u32 = SIO_BASE + 0x14 as u32;
pub const GPIO_OUT_CLR: u32 = SIO_BASE + 0x18 as u32;
pub const GPIO_OUT_XOR: u32 = SIO_BASE + 0x1C as u32;
pub const GPIO_OE_SET:  u32 = SIO_BASE + 0x24 as u32;
pub const GPIO_OE_CLR:  u32 = SIO_BASE + 0x28 as u32;
pub const GPIO_OE_XOR:  u32 = SIO_BASE + 0x2C as u32;

// SysTick Register
pub const SYST_CSR:     u32 = 0xE000_E010;
pub const SYST_RVR:     u32 = 0xE000_E014;
pub const SYST_CVR:     u32 = 0xE000_E018;

pub const SYST_CSR_COUNTFLAG: u32 = 1 << 16;
pub const SYST_CSR_CLKSOURCE: u32 = 1 <<  2;
pub const SYST_CSR_TICKINT:   u32 = 1 <<  1;
pub const SYST_CSR_ENABLE:    u32 = 1 <<  0;

// Clock freq   
pub const CLOCK_XOSC:         usize = 12000000;
pub const CLOCK_REF:          usize = CLOCK_XOSC;

pub const XOSC_MHZ: u32 = 12;
pub const XOSC_KHZ: u32 = XOSC_MHZ * 1000;

pub const TMCLK_MHZ: u32 = 125;
pub const TMCLK_KHZ: u32 = TMCLK_MHZ * 1000;
pub const TIMER_PERIOD:  i32 = 10;

pub const KHZ: u32 = 1000;
pub const MHZ: u32 = KHZ * 1000;

// NVIC Register
pub const SCB_SHPR3:   u32 = 0xE000_ED20;
pub const INTLEVEL_0:  u32 = 0x00;
pub const INTLEVEL_1:  u32 = 0x40;
pub const INTLEVEL_2:  u32 = 0x80;
pub const INTLEVEL_3:  u32 = 0xC0;


