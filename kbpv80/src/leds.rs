use crate::hal;
use hal::gpio::{gpioa::PA5, gpioa::PA6, Disabled, Input, Output, PushPull, AF0};
use hal::hal::digital::OutputPin;
use keyberon::keyboard::Leds as KbLeds;

pub struct Leds {
    caps_led: PA6<Output<PushPull>, AF0>,
    scroll_led: PA5<Output<PushPull>, AF0>,
}

impl Leds {
    pub fn new(sled: PA5<Input<Disabled>, AF0>, cled: PA6<Input<Disabled>, AF0>) -> Self {
        let mut scroll_led = sled.into_output_push_pull();
        let mut caps_led = cled.into_output_push_pull();
        caps_led.set_high().ok();
        scroll_led.set_high().ok();

        Self {
            caps_led,
            scroll_led,
        }
    }
}

impl KbLeds for Leds {
    fn caps_lock(&mut self, status: bool) {
        if status {
            self.caps_led.set_low().ok();
        } else {
            self.caps_led.set_high().ok();
        }
    }

    fn scroll_lock(&mut self, status: bool) {
        if status {
            self.scroll_led.set_low().ok();
        } else {
            self.scroll_led.set_high().ok();
        }
    }
}
