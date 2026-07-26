use super::chip_hal;
use chip_hal::gpio::{Pin, FunctionI2c, PullUp, bank0::{Gpio2, Gpio3}};
use rp2040_hal::fugit::RateExtU32;
use embedded_hal::i2c::I2c;
use cortex_m::delay::Delay;

/// I2C address of the PCF8574 backpack. 0x27 is the common default;
/// some boards ship as 0x3F — check with an i2c scan if `clear()`/`print()` do nothing.
const LCD_ADDR: u8 = 0x27;

// Standard PCF8574 -> HD44780 pin mapping used by nearly all LCD1602 I2C backpacks.
const RS_BIT: u8 = 0b0000_0001;
const EN_BIT: u8 = 0b0000_0100;
const BACKLIGHT_BIT: u8 = 0b0000_1000;

pub struct Screen {
    i2c: chip_hal::I2C<
        chip_hal::pac::I2C1,
        (
            Pin<Gpio2, FunctionI2c, PullUp>,
            Pin<Gpio3, FunctionI2c, PullUp>,
        ),
    >,
    delay: Delay,
    backlight: bool,
}

impl Screen {
    pub(crate) fn new(
        sda: Pin<Gpio2, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
        scl: Pin<Gpio3, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
        i2c_peripheral: chip_hal::pac::I2C1,
        resets: &mut chip_hal::pac::RESETS,
        delay: Delay,
    ) -> Self {

        // Added the 6th argument '125_000_000.Hz()' to satisfy the latest HAL release
        let i2c = chip_hal::I2C::i2c1(
            i2c_peripheral,
            sda.into_pull_up_input().into_function(),
            scl.into_pull_up_input().into_function(),
            400_u32.kHz(),
            resets,
            125_000_000_u32.Hz(), // Standard system bus clock tracking speed
        );

        let mut screen = Screen {
            i2c,
            delay,
            backlight: true,
        };
        screen.init();
        screen
    }

    /// Sends one 4-bit nibble to the display, pulsing the Enable line high then low.
    /// The HD44780 latches data on the falling edge of EN.
    fn write4(&mut self, nibble: u8, rs: bool) {
        let bl = if self.backlight { BACKLIGHT_BIT } else { 0 };
        let rs_bit = if rs { RS_BIT } else { 0 };
        let data = (nibble << 4) | rs_bit | bl;

        let _ = self.i2c.write(LCD_ADDR, &[data | EN_BIT]);
        self.delay.delay_us(1);
        let _ = self.i2c.write(LCD_ADDR, &[data & !EN_BIT]);
        self.delay.delay_us(50);
    }

    /// Sends a full byte as high nibble then low nibble (HD44780 4-bit mode).
    fn write_byte(&mut self, byte: u8, rs: bool) {
        self.write4(byte >> 4, rs);
        self.write4(byte & 0x0F, rs);
    }

    fn command(&mut self, cmd: u8) {
        self.write_byte(cmd, false);
    }

    fn data(&mut self, byte: u8) {
        self.write_byte(byte, true);
    }

    /// Standard HD44780 4-bit power-on init sequence.
    fn init(&mut self) {
        self.delay.delay_ms(50); // wait for LCD Vcc to stabilize

        // Force the controller into a known state, then switch to 4-bit mode.
        self.write4(0x03, false);
        self.delay.delay_ms(5);
        self.write4(0x03, false);
        self.delay.delay_us(150);
        self.write4(0x03, false);
        self.delay.delay_us(150);
        self.write4(0x02, false); // select 4-bit interface
        self.delay.delay_us(150);

        self.command(0x28); // function set: 4-bit, 2 line, 5x8 font
        self.command(0x0C); // display on, cursor off, blink off
        self.command(0x06); // entry mode: auto-increment cursor, no display shift
        self.clear();
    }

    /// Clears all characters on the display and returns the cursor to the home position.
    pub fn clear(&mut self) {
        self.command(0x01);
        self.delay.delay_ms(2); // clear/home need extra settle time (~1.6ms per datasheet)
    }

    pub fn print(&mut self, text: &str) {
        defmt::info!("SDK Console Output: {}", text);
        for byte in text.bytes() {
            self.data(byte);
        }
    }
}