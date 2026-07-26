#![no_std]

pub use cortex_m_rt::entry;
pub use defmt;
pub use rp2040_hal as chip_hal;

pub mod display;
pub mod input;

pub struct PicoPc {
    pub display: display::Screen,
    pub inputs: input::ButtonPad,
}

impl PicoPc {
    pub fn take() -> Self {
        let mut peripherals = chip_hal::pac::Peripherals::take().unwrap();
        let core = cortex_m::Peripherals::take().unwrap();

        let sio = chip_hal::Sio::new(peripherals.SIO);
        let pins = chip_hal::gpio::Pins::new(
            peripherals.IO_BANK0,
            peripherals.PADS_BANK0,
            sio.gpio_bank0,
            &mut peripherals.RESETS,
        );

        // 125 MHz matches the fixed system-clock value passed into I2C::i2c1 in display.rs.
        let delay = cortex_m::delay::Delay::new(core.SYST, 125_000_000);

        let display = display::Screen::new(
            pins.gpio2,
            pins.gpio3,
            peripherals.I2C1,
            &mut peripherals.RESETS,
            delay,
        );

        let inputs = input::ButtonPad::new(
            pins.gpio12,
            pins.gpio13,
            pins.gpio14,
            pins.gpio15,
        );

        PicoPc { display, inputs }
    }
}

pub mod system {
    use super::chip_hal;

    pub fn reset() -> ! {
        cortex_m::interrupt::disable();
        chip_hal::rom_data::reset_to_usb_boot(0, 0);
        loop {
            cortex_m::asm::wfi();
        }
    }
}