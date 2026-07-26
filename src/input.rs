use super::chip_hal;
use chip_hal::gpio::{Pin, FunctionSio, SioInput, PullUp, bank0::{Gpio12, Gpio13, Gpio14, Gpio15}};
use embedded_hal::digital::InputPin;

type Btn1 = Pin<Gpio12, FunctionSio<SioInput>, PullUp>;
type Btn2 = Pin<Gpio13, FunctionSio<SioInput>, PullUp>;
type Btn3 = Pin<Gpio14, FunctionSio<SioInput>, PullUp>;
type Btn4 = Pin<Gpio15, FunctionSio<SioInput>, PullUp>;

pub struct ButtonPad {
    pub btn1: Btn1,
    pub btn2: Btn2,
    pub btn3: Btn3,
    btn4: Btn4,
}

impl ButtonPad {
    pub(crate) fn new(
        b1: Pin<Gpio12, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
        b2: Pin<Gpio13, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
        b3: Pin<Gpio14, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
        b4: Pin<Gpio15, chip_hal::gpio::FunctionNull, chip_hal::gpio::PullDown>,
    ) -> Self {
        ButtonPad {
            btn1: b1.into_pull_up_input(),
            btn2: b2.into_pull_up_input(),
            btn3: b3.into_pull_up_input(),
            btn4: b4.into_pull_up_input(),
        }
    }

    pub fn is_update_held(&mut self) -> bool {
        let pin = &mut self.btn4;
        pin.is_low().unwrap()
    }
}
