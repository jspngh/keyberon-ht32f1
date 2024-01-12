#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_brtt as _;
use ht32f1yyy_hal as hal;
use panic_reset as _;

mod layout;
mod leds;
mod setup;

#[rtic::app(
    device = ht32f1yyy_hal::pac,
    dispatchers = [MCTMBRK, MCTMUP, MCTMTR, MCTMCC],
    peripherals = true
)]
mod app {
    use crate::leds::Leds;
    use crate::setup::{setup, MyMatrix, MyUsbBus, MTRX_C, MTRX_R};
    use core::mem::MaybeUninit;
    use keyberon::debounce::Debouncer;
    use keyberon::key_code::KbHidReport;
    use keyberon::layout::Layout;
    use rtic_monotonics::systick::*;
    use usb_device::bus::UsbBusAllocator;
    use usb_device::class::UsbClass;
    use usb_device::prelude::*;

    #[shared]
    struct Shared {
        keyboard: keyberon::Class<'static, MyUsbBus, Leds>,
    }

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, MyUsbBus>,
        matrix: MyMatrix,
        debouncer: Debouncer<[[bool; MTRX_C]; MTRX_R]>,
        layout: Layout<MTRX_C, MTRX_R, 2, ()>,
    }

    #[init(local = [
        usb_bus: MaybeUninit<UsbBusAllocator<MyUsbBus>> = MaybeUninit::uninit(),
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let (keyboard, usb_dev, matrix) = setup(cx.device, cx.core, cx.local.usb_bus);
        let debouncer = Debouncer::new([[false; MTRX_C]; MTRX_R], [[false; MTRX_C]; MTRX_R], 5);

        tick::spawn().ok();

        (
            Shared { keyboard },
            Local {
                usb_dev,
                matrix,
                debouncer,
                layout: Layout::new(&crate::layout::LAYERS),
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task(priority = 1, shared = [keyboard], local = [matrix, layout, debouncer])]
    async fn tick(mut cx: tick::Context) {
        loop {
            for event in cx.local.debouncer.events(cx.local.matrix.get().unwrap()) {
                cx.local.layout.event(event);
            }
            match cx.local.layout.tick() {
                keyberon::layout::CustomEvent::Release(()) => {
                    // TODO
                }
                _ => (),
            }
            let report: KbHidReport = cx.local.layout.keycodes().collect();
            if cx
                .shared
                .keyboard
                .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
            {
                while let Ok(0) = cx.shared.keyboard.lock(|k| k.write(report.as_bytes())) {}
            }

            Systick::delay(1.millis()).await;
        }
    }

    #[task(priority = 8, binds = USB, local = [usb_dev], shared = [keyboard])]
    fn on_usb(mut cx: on_usb::Context) {
        cx.shared.keyboard.lock(|keyboard| {
            if cx.local.usb_dev.poll(&mut [keyboard]) {
                keyboard.poll();
            }
        });
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
